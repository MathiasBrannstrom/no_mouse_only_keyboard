use std::{collections::HashMap, hash::Hash, process, thread::sleep, time::Duration};

use inputbot::{KeybdKey::{self, *}, MouseButton, MouseCursor};

#[derive(Eq, PartialEq, Hash)]
enum Action {
    MouseMove(Direction),
    MouseClick(MouseButton),
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
    keybinds.insert(Action::SpeedUp, F22Key);
    keybinds.insert(Action::SpeedDown, F21Key);


    start_loop(&keybinds)

}

fn start_loop(keybinds: &HashMap<Action, KeybdKey>) {

    const FAST_SPEED:i32 = 10;
    const SLOW_SPEED:i32 = 1;
    const DEFAULT_SPEED:i32 = 3;

    let mut speed_arc;


    loop {
        speed_arc = DEFAULT_SPEED;
        
        if F10Key.is_pressed() {
            process::exit(0);
        }

        match keybinds.get(&Action::SpeedUp) {
            Some(key) => {
                if key.is_pressed() {
                    speed_arc = FAST_SPEED;
                }
            },
            None => { }
        }

        match keybinds.get(&Action::SpeedDown) {
            Some(key) => {
                if key.is_pressed() {
                    speed_arc = SLOW_SPEED;
                }
            },
            None => { }
        }

        for direction in Direction::iter() {
            match keybinds.get(&Action::MouseMove(direction)) {
                Some(key) => {
                    if key.is_pressed() {
                        let (x,y) = direction.into_i32s();
                        MouseCursor::move_rel(x*speed_arc , y*speed_arc);
                    }
                },
                None => { }
            }
        }

        sleep(Duration::from_millis(10))
    }
}