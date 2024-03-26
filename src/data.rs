use inputbot::MouseButton;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Action {
    MouseMove(Direction),
    MouseClick(MouseButton),
    MouseScroll(Direction),
    SpeedUp,
    SpeedDown,
    ShowGridNavigation,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Direction {
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