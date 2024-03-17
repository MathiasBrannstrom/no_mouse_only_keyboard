use std::{borrow::Borrow, collections::HashMap, hash::Hash, process, sync::Arc, thread::sleep, time::Duration};

use inputbot::{KeybdKey::{self, *}, MouseButton, MouseCursor};

#[derive(Eq, PartialEq, Hash)]
enum Action {
    MouseMoveUp,
    MouseMoveDown,
    MouseMoveLeft,
    MouseMoveRight,

    // MouseLeftClick,
}

fn main() {
    
    let speed_arc = Arc::new(3);

    let mut keybinds = HashMap::new();

    keybinds.insert(F13Key, Action::MouseMoveUp);
    keybinds.insert(F14Key, Action::MouseMoveLeft);
    keybinds.insert(F15Key, Action::MouseMoveDown);
    keybinds.insert(F23Key, Action::MouseMoveRight);

    start_loop(&keybinds, speed_arc)

}

fn start_loop(keybinds: &HashMap<KeybdKey, Action>, speed_arc: Arc<i32>) {

    let actions = create_actions_hashmap(speed_arc);
    loop {
        
        if F10Key.is_pressed() {
            process::exit(0);
        }
        for (key, mouse_move) in keybinds {
            if key.is_pressed() {
                if let Some(action) = actions.get(mouse_move) {
                    action();
                }
                
            }
        }

        sleep(Duration::from_millis(10))
    }
}

fn create_actions_hashmap(speed_arc: Arc<i32>) -> HashMap<Action, Box<dyn Fn()>> {
    let mut actions: HashMap<Action, Box<dyn Fn()>> = HashMap::new();

    let speed_clone = speed_arc.clone();
    actions.insert(Action::MouseMoveUp, Box::new(move || MouseCursor::move_rel(0, -*speed_clone)));
    
    let speed_clone = speed_arc.clone();
     actions.insert(Action::MouseMoveLeft, Box::new(move || MouseCursor::move_rel( -*speed_clone, 0)));
    
     let speed_clone = speed_arc.clone();
    actions.insert(Action::MouseMoveDown, Box::new(move || MouseCursor::move_rel(0, *speed_clone)));
    
    let speed_clone = speed_arc.clone();
    actions.insert(Action::MouseMoveRight, Box::new(move || MouseCursor::move_rel(*speed_clone, 0)));

    // actions.insert(Action::MouseLeftClick, Box::new(|| {
    //     MouseButton::LeftButton.press();
    //     MouseButton::LeftButton.release();
    // }
    // ));
    
    actions
}

// fn setup_action(actions_map: &mut HashMap<Action, Box<dyn Fn()>>, action:Action, closure: Box<dyn Fn()>, speed_arc: Arc<i32>) {
//     actions_map.insert(action, v)
// }