use std::{collections::HashMap, process, thread::sleep, time::Duration};
use inputbot::{KeybdKey::{self, *}, MouseButton, MouseCursor, MouseWheel};

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

mod data;
use data::{Action, Direction};

mod grid_navigation {
    pub mod ui;
}

use grid_navigation::ui::BasicApp;

fn main() {


    let keybinds = vec![
        (Action::MouseMove(Direction::Up), F13Key),
        (Action::MouseMove(Direction::Left), F14Key),
        (Action::MouseMove(Direction::Down), F15Key),
        (Action::MouseMove(Direction::Right), F23Key),
        (Action::SpeedUp, F21Key),
        (Action::SpeedDown, F22Key),
        (Action::MouseClick(MouseButton::LeftButton), F17Key),
        (Action::MouseClick(MouseButton::RightButton), F18Key),
        (Action::MouseScroll(Direction::Up), F19Key),
        (Action::MouseScroll(Direction::Down), F20Key),
        (Action::ShowGridNavigation, F24Key)
    ].into_iter().collect::<HashMap<Action, KeybdKey>>();

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

    let mut mouse_click_state: HashMap<MouseButton, bool> = HashMap::from([ // Should technically be initialized to button state.
        (MouseButton::LeftButton, false), 
        (MouseButton::MiddleButton, false),
        (MouseButton::RightButton, false)]
    );

    let mut show_grid_navigation_button_pressed = false; // Should be initialized to button state.

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

        update_grid_navigation_state(&mut show_grid_navigation_button_pressed, &keybinds);

        sleep(Duration::from_millis(5))
    }
}

fn update_grid_navigation_state(show_grid_navigation_button_pressed: &mut bool, keybinds: &HashMap<Action, KeybdKey>) {
    match keybinds.get(&Action::ShowGridNavigation) {
        Some(key) => {
            if key.is_pressed() {
                if !*show_grid_navigation_button_pressed {
                    
                    show_grid_navigation_ui();
                }
                *show_grid_navigation_button_pressed = true;
            } else {
                *show_grid_navigation_button_pressed = false;
            }
        },
        None => todo!(),
    }
}


fn show_grid_navigation_ui() {
    
    BasicApp::show_app();
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