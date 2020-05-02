use crate::position::Position;
use rand::Rng;

pub const BOARD_WIDTH: usize = 4;
pub const BOARD_HEIGHT: usize = 4;
const BLANK_NUMBER: usize = 16;
const SHUFFLE_COUNT: usize = 10;

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

    //シャッフル付き初期化
    pub fn initialize() -> Self {
        let mut board = Board::new();
        board.shuffle();
        board
    }

    //シャッフル
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..SHUFFLE_COUNT {
            let i = rng.gen_range(1, 101);
            let blank_position = self.blank_position();
            let blank_position = match i % 4 {
                0 => blank_position.plus_y(),
                1 => blank_position.minus_y(),
                2 => blank_position.plus_x(),
                3 => blank_position.minus_x(),
                _ => blank_position
            };
            self.move_blank(blank_position);
        }
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
        //クリアしたかどうか
        if self.is_finished() {
            output = format!("{}CONGRATULATION!", output);
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

    //ブランク位置の変更
    pub fn move_blank(&mut self, move_position: Position) {
        let blank_position = self.blank_position();
        self.cells[blank_position.y()][blank_position.x()] = self.cells[move_position.y()][move_position.x()];
        self.cells[move_position.y()][move_position.x()] = BLANK_NUMBER;
    }

    //ゲームクリア判定
    fn is_finished(&self) -> bool {
        let mut is_finished = true;
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if self.cells[y][x] != y * BOARD_WIDTH + x + 1 {
                    is_finished = false;
                    break;
                }
            }
        }
        is_finished
    }
}
