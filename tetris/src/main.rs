use pancurses::{initscr, noecho, endwin};
use tetris::board::Board;
use std::thread::sleep;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    //入力受付
    let device_state = DeviceState::new();
    let window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();

    //CTRL+Cで抜ける
    ctrlc::set_handler(move || {
        endwin();
        std::process::exit(exitcode::OK);
    }).expect("Error setting Ctrl-C handler");

    //初期表示
    let mut board = Board::initialize();
    window.printw(board.output());
    window.refresh();

    loop {
        //入力受付
        let keys: Vec<Keycode> = device_state.get_keys();
        if !keys.is_empty() {
            match keys[0] {
                Keycode::Down => board.down(),
                Keycode::Left => board.left(),
                Keycode::Right => board.right(),
                _ => board.rotate(),
            };

            //再描画
            window.clear();
            window.printw(board.output());
            window.refresh();
            sleep(Duration::from_millis(100));
        }
    }
}
