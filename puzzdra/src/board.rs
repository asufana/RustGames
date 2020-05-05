use rand::Rng;

const BOARD_HEIGHT: usize = 6;
const BOARD_WIDTH: usize = 5;
const DROP_MAX: usize = 5;

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
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                this.set_cell(x, y, Board::random_value());
            }
        }
        this
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
                output = format!("{}{: >2}", output, n);
            }
            output = format!("{}\n", output);
        }
        format!("{}\n", output)
    }
}
