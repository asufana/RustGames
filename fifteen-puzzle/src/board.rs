use crate::position::Position;

pub const BOARD_WIDTH: usize = 4;
pub const BOARD_HEIGHT: usize = 4;
const BLANK_NUMBER: usize = 16;

//盤面
pub struct Board { cells: [[usize; BOARD_HEIGHT]; BOARD_WIDTH] }

impl Board {
    //初期化
    pub fn new() -> Self {
        let mut cells = [[0; BOARD_HEIGHT]; BOARD_WIDTH];
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                cells[y][x] = y * BOARD_WIDTH + x + 1;
            }
        }
        Self { cells }
    }

    //表示
    pub fn output(&self) -> String {
        let mut output = String::new();
        output = format!("{}+--+--+--+--+\n", output);
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if self.cells[y][x] == BLANK_NUMBER {
                    output = format!("{}|  ", output);
                } else {
                    output = format!("{}|{: >2}", output, self.cells[y][x]);
                }
            }
            output = format!("{}|\n+--+--+--+--+\n", output);
        }
        output
    }

    //ブランク位置の取得
    pub fn blank_position(&self) -> Position {
        let mut blank_position = Position::empty();
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if self.cells[y][x] == BLANK_NUMBER {
                    blank_position = Position::new(x, y);
                }
            }
        }
        blank_position
    }
}
