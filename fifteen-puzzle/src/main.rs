use pancurses::{initscr, Input, noecho};
use fifteen_puzzle::board::Board;

fn main() {
    //入力受付
    let window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();

    //初期表示
    let mut board = Board::initialize();
    window.printw(board.output());

    loop {
        //ブランク位置
        let blank_position = board.blank_position();

        //入力受付
        match window.getch() {
            //カーソル位置移動
            Some(Input::KeyUp) => board.move_blank(blank_position.minus_y()),
            Some(Input::KeyDown) => board.move_blank(blank_position.plus_y()),
            Some(Input::KeyLeft) => board.move_blank(blank_position.minus_x()),
            Some(Input::KeyRight) => board.move_blank(blank_position.plus_x()),
            _ => ()
        };

        //再描画
        window.clear();
        window.printw(board.output());
    }
}