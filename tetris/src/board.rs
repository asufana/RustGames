use crate::mino::{MINO_WIDTH, MINO_HEIGHT, Mino};

pub const BOARD_HEIGHT: usize = 14;
pub const BOARD_WIDTH: usize = 12;

pub struct Board {
    field: [[usize; BOARD_WIDTH]; BOARD_HEIGHT],
    buffer: [[usize; BOARD_WIDTH + MINO_WIDTH]; BOARD_HEIGHT + MINO_HEIGHT],
    mino: Mino,
}

impl Board {
    //ボードの全フィールドに対して何らか処理を行う
    fn apply_fields<F>(&mut self, mut function: F)
        where F: FnMut(&mut Board, usize, usize) -> () {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                function(self, x, y);
            }
        }
    }
    //ミノの全フィールドに対して何らか処理を行う
    fn apply_mino_fields<F>(&mut self, mut function: F)
        where F: FnMut(&mut Board, usize, usize) -> () {
        for y in 0..MINO_HEIGHT {
            for x in 0..MINO_WIDTH {
                function(self, x, y);
            }
        }
    }

    fn new() -> Self {
        Self {
            field: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
            buffer: [[0; BOARD_WIDTH + MINO_WIDTH]; BOARD_HEIGHT + MINO_HEIGHT],
            mino: Mino::random(),
        }
    }

    //初期化
    pub fn initialize() -> Self {
        let mut this = Board::new();

        //枠描画
        this.apply_fields(|board: &mut Board, x: usize, y: usize| {
            if Board::is_frame(x, y) {
                board.set_field(x, y, 1);
            }
        });

        //ミノ描画
        this.apply_mino_fields(|board: &mut Board, x: usize, y: usize| {
            board.set_buffer_field(x + board.mino.x, y + board.mino.y, board.mino.value(x, y));
        });
        this
    }

    //枠位置かどうか
    fn is_frame(x: usize, y: usize) -> bool {
        y == 0 || y == BOARD_HEIGHT - 1 || x == 0 || x == BOARD_WIDTH - 1
    }

    //アクセサ
    fn get_field(&self, x: usize, y: usize) -> usize { self.field[y][x] }
    fn set_field(&mut self, x: usize, y: usize, value: usize) { self.field[y][x] = value; }
    fn get_buffer_field(&self, x: usize, y: usize) -> usize { self.buffer[y][x] }
    fn set_buffer_field(&mut self, x: usize, y: usize, value: usize) { self.buffer[y][x] = value; }

    //描画
    pub fn output(&mut self) -> String {
        let mut output = String::new();
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let n = self.get_field(x, y) + self.get_buffer_field(x, y);
                output = if n != 0 {
                    format!("{}🔳", output)
                } else {
                    format!("{}　", output)
                };
            }
            output = format!("{}\n", output);
        }
        format!("{}\n", output)
    }
}