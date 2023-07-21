use ncurses::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

fn main() {
    let window = initscr();

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    addstr("Rust Todo List");
    refresh();

    let mut quit = false;
    let mut todos = vec!["Learn Rust", "Learn Nix", "Learn Vim Motions", "Buy a bread"];
    let todo_curr: usize = 0;

    while !quit {
    
       for (idx, todo) in todos.iter().enumerate() {
           let pair = {
                if todo_curr == index {
                    return HIGHLIGHT_PAIR
                } else {
                    return REGULAR_PAIR
                }
           }

            mv(idx as i32, 0);
            addstr(*todo);
       }

       refresh();

       let key = getch();
       match key as u8 as char {
           'q' => quit = true,
            _ => {}
       }
    }

    endwin();
}
