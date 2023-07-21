use pancurses::{endwin, initscr, set_title, Input};

fn main() {
    let window = initscr();
    set_title("Rust Todo List");
    window.refresh();

    let mut quit = false;

    while !quit {
        let user_input = window.getch();
        match user_input {
            Some(Input::Character(c)) => {
                window.addch(c);
            }
            _ => {}
        }
    }

    endwin();
}
