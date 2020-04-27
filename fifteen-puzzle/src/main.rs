use pancurses::{initscr, Input, noecho};
use fifteen_puzzle::board::Board;

fn main() {
    //入力受付
    let window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();

    //初期表示
    let board = Board::new();
    window.printw(board.output());

    loop {
        //ブランク位置
        let blank_position = board.blank_position();
        window.printw(format!("BLANK: y:{}, x:{}", blank_position.y(), blank_position.x()));

        //入力受付
        match window.getch() {
            Some(Input::KeyUp) => println!("↑"),
            Some(Input::KeyDown) => println!("↓"),
            Some(Input::KeyLeft) => println!("←"),
            Some(Input::KeyRight) => println!("→"),
            _ => ()
        };

        //再描画
        window.clear();
        window.printw(board.output());
    }
}