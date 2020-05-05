use rand::Rng;
use crate::drop_type::{DROP_MAX, DropType};
use crate::position::Position;
use crate::board::Direction::{RIGHT, DOWN};

pub const BOARD_HEIGHT: usize = 6;
pub const BOARD_WIDTH: usize = 5;
const ERASE_CHAIN_COUNT: i8 = 3;

#[derive(Copy, Clone)]
enum Direction {
    RIGHT,
    DOWN,
}

impl Direction {
    pub fn iterator() -> impl Iterator<Item=Direction> {
        [RIGHT, DOWN].iter().copied()
    }
}

pub struct Board {
    cells: [[usize; BOARD_WIDTH]; BOARD_HEIGHT],
    erase_cells: [[bool; BOARD_WIDTH]; BOARD_HEIGHT],
    pub cursor: Position,
    holding: bool,
}

impl Board {
    fn new() -> Self {
        Self {
            cells: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
            erase_cells: [[false; BOARD_WIDTH]; BOARD_HEIGHT],
            cursor: Position::empty(),
            holding: false,
        }
    }

    //初期化
    pub fn initialize() -> Self {
        let mut this = Board::new();
        this.apply_cells(|board: &mut Board, x: usize, y: usize| {
            board.set_cell(x, y, Board::random_value());
        });
        this
    }

    //ボードの全セルに対して何らか処理を行う
    fn apply_cells<F>(&mut self, mut function: F)
        where F: FnMut(&mut Board, usize, usize) -> () {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                function(self, x, y);
            }
        }
    }

    //ランダム値の生成
    fn random_value() -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(1, 101) % DROP_MAX + 1
    }

    //アクセサ
    fn get_cell(&self, x: usize, y: usize) -> usize { self.cells[y][x] }
    fn set_cell(&mut self, x: usize, y: usize, value: usize) { self.cells[y][x] = value; }
    fn get_erase_cell(&self, x: usize, y: usize) -> bool { self.erase_cells[y][x] }
    fn set_erase_cell(&mut self, x: usize, y: usize, value: bool) { self.erase_cells[y][x] = value; }

    //ホールドと解除
    pub fn hold(&mut self) {
        self.holding = !self.holding;

        if !self.holding && self.has_chain() {
            self.erase_drops();
        }
    }

    //ブランクかどうか
    fn is_blank(&self, x: usize, y: usize) -> bool {
        self.get_cell(x, y) == 0
    }

    //カーソル移動によるセルの入れ替え
    pub fn move_cursor(&mut self, cursor: Position) {
        if self.holding {
            let current_cursor = self.cursor;
            let current_value = self.get_cell(current_cursor.x(), current_cursor.y());
            self.set_cell(current_cursor.x(), current_cursor.y(), self.get_cell(cursor.x(), cursor.y()));
            self.set_cell(current_cursor.x(), current_cursor.y(), self.get_cell(cursor.x(), cursor.y()));
            self.set_cell(cursor.x(), cursor.y(), current_value);
        }
        self.cursor = cursor;
    }

    //連続したドロップが存在するかどうか
    fn has_chain(&mut self) -> bool {
        let mut has_chain = false;
        self.apply_cells(|board: &mut Board, x: usize, y: usize| {
            for d in Direction::iterator() {
                let chain_count = board.check_chain(x, y, d, false);
                if chain_count >= ERASE_CHAIN_COUNT {
                    has_chain = true;
                }
            }
        });
        has_chain
    }

    //ドロップ連続値の取得
    fn check_chain(&mut self, x: usize, y: usize, dir: Direction, erase: bool) -> i8 {
        if self.is_blank(x, y) {
            return 0;
        };
        if erase {
            self.set_erase_cell(x, y, true);
        }

        let mut count: i8 = 1;
        let mut next_position = match dir {
            Direction::RIGHT => Position::new(x, y).get_right_position(),
            Direction::DOWN => Position::new(x, y).get_down_position(),
        };

        loop {
            if !next_position.within_limit() {
                break;
            }
            if self.get_cell(x, y) != self.get_cell(next_position.x(), next_position.y()) {
                break;
            }
            if erase {
                self.set_erase_cell(next_position.x(), next_position.y(), true);
            }

            count += 1;
            next_position = match dir {
                Direction::RIGHT => next_position.get_right_position(),
                Direction::DOWN => next_position.get_down_position(),
            };
        }
        count
    }

    //削除処理
    fn erase_drops(&mut self) {
        //削除フラグの設定
        self.apply_cells(|board: &mut Board, x: usize, y: usize| {
            for d in Direction::iterator() {
                let chain_count = board.check_chain(x, y, d, false);
                if chain_count >= ERASE_CHAIN_COUNT {
                    board.check_chain(x, y, d, true);
                }
            }
        });
        //削除フラグに基づいて削除
        self.apply_cells(|board: &mut Board, x: usize, y: usize| {
            if board.get_erase_cell(x, y) == true {
                board.set_cell(x, y, 0);
            }
        });
        //削除フラグのクリア
        self.erase_cells = [[false; BOARD_WIDTH]; BOARD_HEIGHT];
    }

    //描画
    pub fn output(&self) -> String {
        let mut output = String::new();
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let n = self.get_cell(x, y);
                let drop_type = DropType::from(n);
                let aa = if self.holding && y == self.cursor.y() && x == self.cursor.x() {
                    drop_type.to_aa_selected()
                } else {
                    drop_type.to_aa()
                };
                output = if drop_type == DropType::NONE {
                    format!("{}{}", output, aa)
                } else {
                    format!("{}{: >2}", output, aa)
                }
            }
            if y == self.cursor.y() {
                output = format!("{} 👈", output);
            } else {
                output = format!("{}　", output);
            }
            output = format!("{}\n", output);
        }
        for x in 0..BOARD_WIDTH {
            if x == self.cursor.x() {
                output = format!("{} 👆", output);
            } else {
                output = format!("{}　", output);
            }
        }
        format!("{}\n", output)
    }
}
