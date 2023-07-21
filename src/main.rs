use pancurses::{endwin, initscr, set_title};

fn main() {
    let window = initscr();
    set_title("Rust Todo List");
    window.refresh();

    window.getch();
    endwin();
}
