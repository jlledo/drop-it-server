use enigo::{Enigo, MouseButton, MouseControllable};

#[derive(Debug, Eq, PartialEq)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Move(Vector2),
    LeftClick,
    RightClick,
    Scroll(Vector2),
}

impl Command {
    pub fn execute(&self) {
        let mut enigo = Enigo::new();
        match self {
            Command::Move(vector) => enigo.mouse_move_relative(vector.x, vector.y),
            Command::LeftClick => enigo.mouse_click(MouseButton::Left),
            Command::RightClick => enigo.mouse_click(MouseButton::Right),
            Command::Scroll(vector) => {
                enigo.mouse_scroll_x(vector.x);
                enigo.mouse_scroll_y(vector.y);
            }
        }
    }
}
