use pancurses::{initscr, Input, noecho, endwin};

fn main() {
    //入力受付
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
    window.printw("test");

    loop {
        //入力受付
        match window.getch() {
            Some(Input::KeyUp) => println!("↑"),
            Some(Input::KeyDown) => println!("↓"),
            Some(Input::KeyLeft) => println!("←"),
            Some(Input::KeyRight) => println!("→"),
            _ => ()
        };

        //再描画
        //window.clear();
        //window.printw("test");
    }
}
