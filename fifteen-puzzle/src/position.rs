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

#[cfg(test)]
mod test {
    use crate::board::{BOARD_WIDTH, BOARD_HEIGHT};
    use crate::position::Position;

    #[test]
    fn test_move_position() {
        //加算減算が正しく処理されること
        let position = Position::new(1, 1);
        assert_eq!(Position::new(0, 1), position.minus_x());
        assert_eq!(Position::new(2, 1), position.plus_x());
        assert_eq!(Position::new(1, 0), position.minus_y());
        assert_eq!(Position::new(1, 2), position.plus_y());

        //ゼロより小さくならないこと
        let position = Position::new(0, 0);
        assert_eq!(Position::new(0, 0), position.minus_x());
        assert_eq!(Position::new(0, 0), position.minus_y());

        //最大サイズより大きくならないこと
        let position = Position::new(BOARD_WIDTH, BOARD_HEIGHT);
        assert_eq!(Position::new(BOARD_WIDTH, BOARD_HEIGHT), position.plus_x());
        assert_eq!(Position::new(BOARD_WIDTH, BOARD_HEIGHT), position.plus_y());
    }
}

