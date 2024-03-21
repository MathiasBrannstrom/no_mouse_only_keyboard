use std::{collections::HashMap, hash::Hash, process, thread::sleep, time::Duration};

use inputbot::{KeybdKey::{self, *}, MouseButton, MouseCursor, MouseWheel};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Action {
    MouseMove(Direction),
    MouseClick(MouseButton),
    MouseScroll(Direction),
    SpeedUp,
    SpeedDown
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right].iter().copied()
    }

    pub fn into_i32s(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        }
    }
}

fn main() {
    
    

    let mut keybinds = HashMap::new();

    keybinds.insert(Action::MouseMove(Direction::Up), F13Key);
    keybinds.insert(Action::MouseMove(Direction::Left), F14Key);
    keybinds.insert(Action::MouseMove(Direction::Down), F15Key);
    keybinds.insert(Action::MouseMove(Direction::Right), F23Key);
    keybinds.insert(Action::SpeedUp, F21Key);
    keybinds.insert(Action::SpeedDown, F22Key);
    keybinds.insert(Action::MouseClick(MouseButton::LeftButton), F17Key);
    keybinds.insert(Action::MouseClick(MouseButton::RightButton), F18Key);
    keybinds.insert(Action::MouseScroll(Direction::Up), F19Key);
    keybinds.insert(Action::MouseScroll(Direction::Down), F20Key);


    start_loop(&keybinds);
}

fn start_loop(keybinds: &HashMap<Action, KeybdKey>) {

    const FAST_MOUSE_SPEED:i32 = 20;
    const SLOW_MOUSE_SPEED:i32 = 1;
    const DEFAULT_MOUSE_SPEED:i32 = 5;

    const FAST_SCROLL_SPEED:i32 = 5;
    const SLOW_SCROLL_SPEED:i32 = 1;
    const DEFAULT_SCROLL_SPEED:i32 = 2;


    let mut mouse_speed;
    let mut scroll_speed;

    let mut mouse_click_state: HashMap<MouseButton, bool> = HashMap::from([
        (MouseButton::LeftButton, false),
        (MouseButton::MiddleButton, false),
        (MouseButton::RightButton, false)]
    );

    loop {
        mouse_speed = DEFAULT_MOUSE_SPEED;
        scroll_speed = DEFAULT_SCROLL_SPEED;
        
        if F10Key.is_pressed() {
            process::exit(0);
        }

        match keybinds.get(&Action::SpeedUp) {
            Some(key) => {
                if key.is_pressed() {
                    mouse_speed = FAST_MOUSE_SPEED;
                    scroll_speed = FAST_SCROLL_SPEED;
                }
            },
            None => { }
        }

        match keybinds.get(&Action::SpeedDown) {
            Some(key) => {
                if key.is_pressed() {
                    mouse_speed = SLOW_MOUSE_SPEED;
                    scroll_speed = SLOW_SCROLL_SPEED;
                }
            },
            None => { }
        }

        update_mouse_state(MouseButton::LeftButton, keybinds, &mut mouse_click_state);
        update_mouse_state(MouseButton::RightButton, keybinds, &mut mouse_click_state);

        for direction in Direction::iter() {
            match keybinds.get(&Action::MouseMove(direction)) {
                Some(key) => {
                    if key.is_pressed() {
                        let (x,y) = direction.into_i32s();
                        MouseCursor::move_rel(x*mouse_speed , y*mouse_speed);
                    }
                },
                None => { }
            }
        }

        for direction in Direction::iter() {
            match keybinds.get(&Action::MouseScroll(direction)) {
                Some(key) => {
                    if key.is_pressed() {
                        let (x,y) = direction.into_i32s();

                        if direction == Direction::Down || direction == Direction::Up {
                            MouseWheel::scroll_ver(-y*scroll_speed);
                        } else {
                            MouseWheel::scroll_hor(-x*scroll_speed);

                        }
                    }
                },
                None => { }
            }
        }

        sleep(Duration::from_millis(5))
    }
}

fn update_mouse_state(button: MouseButton, keybinds: &HashMap<Action, KeybdKey>, mouse_button_states: &mut HashMap<MouseButton, bool>) {
    
    match keybinds.get(&Action::MouseClick(button)) {
        Some(key) => {
            if key.is_pressed() {
                if !mouse_button_states.get(&button).expect("Mouse button was not added to HashMap") {
                    mouse_button_states.insert(button, true);
                    MouseButton::press(button);
                }
            } else {
                if *mouse_button_states.get(&button).expect("Mouse button was not added to HashMap") {
                    mouse_button_states.insert(button, false);
                    MouseButton::release(button);
                }
            }
        },
        None => todo!(),
    }
}