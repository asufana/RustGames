use pancurses::{initscr, Input, noecho};

fn main() {
    //入力受付
    let window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();

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
