use crate::board::{BOARD_HEIGHT, BOARD_WIDTH};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position { x: usize, y: usize }

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn empty() -> Self {
        Position::new(0, 0)
    }
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
    pub fn plus_x(&self) -> Self {
        if self.x >= BOARD_WIDTH - 1 {
            *self
        } else {
            Position::new(self.x + 1, self.y)
        }
    }
    pub fn minus_x(&self) -> Self {
        if self.x == 0 {
            *self
        } else {
            Position::new(self.x - 1, self.y)
        }
    }
    pub fn plus_y(&self) -> Self {
        if self.y >= BOARD_HEIGHT - 1 {
            *self
        } else {
            Position::new(self.x, self.y + 1)
        }
    }
    pub fn minus_y(&self) -> Self {
        if self.y == 0 {
            *self
        } else {
            Position::new(self.x, self.y - 1)
        }
    }
}

