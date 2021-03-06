use crate::board::{BOARD_HEIGHT, BOARD_WIDTH};

#[derive(Copy, Clone)]
pub struct Position {
    x: usize,
    y: usize
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self { Self { x, y } }
    pub fn empty() -> Self { Position::new(0, 0) }
    pub fn x(&self) -> usize { self.x }
    pub fn y(&self) -> usize { self.y }

    //カーソル位置の移動
    pub fn move_plus_y(&self) -> Self {
        if self.y == BOARD_HEIGHT - 1 {
            *self
        } else {
            Position::new(self.x, self.y + 1)
        }
    }
    //カーソル位置の移動
    pub fn move_minus_y(&self) -> Self {
        if self.y == 0 {
            *self
        } else {
            Position::new(self.x, self.y - 1)
        }
    }
    //カーソル位置の移動
    pub fn move_plus_x(&self) -> Self {
        if self.x == BOARD_WIDTH - 1 {
            *self
        } else {
            Position::new(self.x + 1, self.y)
        }
    }
    //カーソル位置の移動
    pub fn move_minus_x(&self) -> Self {
        if self.x == 0 {
            *self
        } else {
            Position::new(self.x - 1, self.y)
        }
    }

    //一つ右に移動
    pub fn get_right_position(&self) -> Self {
        Position::new(self.x + 1, self.y)
    }
    //一つ下に移動
    pub fn get_down_position(&self) -> Self {
        Position::new(self.x, self.y + 1)
    }
    //枠内かどうか
    pub fn within_limit(&self) -> bool {
        //右移動と下移動しかないため最大値のみチェックする
        self.y <= BOARD_HEIGHT - 1 && self.x <= BOARD_WIDTH - 1
    }
}
