use pancurses::{initscr, Input, noecho};
use puzzdra::board::Board;
use std::thread::sleep;
use std::time::Duration;

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
        //入力受付
        match window.getch() {
            Some(Input::KeyUp) => board.move_cursor(board.cursor.move_minus_y()),
            Some(Input::KeyDown) => board.move_cursor(board.cursor.move_plus_y()),
            Some(Input::KeyLeft) => board.move_cursor(board.cursor.move_minus_x()),
            Some(Input::KeyRight) => board.move_cursor(board.cursor.move_plus_x()),
            _ => board.hold()
        };

        //再描画
        loop {
            window.clear();
            window.printw(board.output());
            window.refresh();

            //アニメーション処理中であれば処理し続ける
            if !board.animating() {
                sleep(Duration::from_millis(300));
                board.erase_and_animate();
            } else {
                break;
            }
        }
    }
} 