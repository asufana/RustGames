use rand::Rng;
use crate::drop_type::{DROP_MAX, DropType};

const BOARD_HEIGHT: usize = 6;
const BOARD_WIDTH: usize = 5;

pub struct Board {
    cells: [[usize; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    fn new() -> Self {
        Self {
            cells: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
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

    //描画
    pub fn output(&self) -> String {
        let mut output = String::new();
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let n = self.get_cell(x, y);
                let drop_type = DropType::from(n);
                let aa = drop_type.to_aa();
                output = format!("{}{: >2}", output, aa);
            }
            output = format!("{}　\n", output);
        }
        format!("{}\n", output)
    }
}
