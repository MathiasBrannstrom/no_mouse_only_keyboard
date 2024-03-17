use std::{collections::HashMap, hash::Hash, process, sync::Arc, thread::sleep, time::Duration};

use inputbot::{KeybdKey::{self, *}, MouseCursor};

#[derive(Eq, PartialEq, Hash)]
enum MouseMove {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    
    let speed_arc = Arc::new(3);

    let mut keybinds = HashMap::new();

    keybinds.insert(F13Key, MouseMove::Up);
    keybinds.insert(F14Key, MouseMove::Left);
    keybinds.insert(F15Key, MouseMove::Down);
    keybinds.insert(F23Key, MouseMove::Right);

    start_loop(&keybinds, speed_arc)

}

fn start_loop(keybinds: &HashMap<KeybdKey, MouseMove>, speed_arc: Arc<i32>) {

    let directions = create_directions_hashmap();
    loop {
        
        if F10Key.is_pressed() {
            process::exit(0);
        }
        for (key, mouse_move) in keybinds {
            if key.is_pressed() {
                let (x,y) = directions.get(mouse_move).unwrap();
                MouseCursor::move_rel(*x * *speed_arc, *y * *speed_arc);
            }
        }

        sleep(Duration::from_millis(10))
    }
}

fn create_directions_hashmap() -> HashMap<MouseMove, (i32, i32)> {
    let mut directions = HashMap::new();

    directions.insert(MouseMove::Up, (0, -1));
    directions.insert(MouseMove::Left, (-1, 0));
    directions.insert(MouseMove::Down, (0, 1));
    directions.insert(MouseMove::Right, (1, 0));
    directions
}

// fn set_up_key_for_direction(key: KeybdKey, dx: i32, dy: i32, speed: Arc<i32>) {
    
//     let is_pressed = Arc::new(Mutex::new(false));

//     key.bind(move || {
//         let mut is_pressed = is_pressed.lock().unwrap();

//         if *is_pressed {
//             return;
//         }

//         while key.is_pressed() {
//             *is_pressed = true;
//             MouseCursor::move_rel(dx* *speed, dy* *speed);
//             sleep(Duration::from_millis(10));
//         }
//         *is_pressed = false;
//     });

// }


