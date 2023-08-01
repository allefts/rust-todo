use ncurses::*;
use std::cmp::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

type Id = usize;

#[derive(Default)]
struct UI {
    list_curr: Option<Id>;
}

impl UI {
    fn begin_list(&mut self, id: usize) {
        todo!();
    }

    fn list_element(label: &str, id: usize) {
        todo!();
    }

    fn end_list(&mut self) {
        todo!();
    }
}

fn main() {
    let window = initscr();

    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    //addstr("Rust Todo List");
    refresh();

    let mut quit = false;
    let mut todos = vec!["Learn Rust", "Learn Nix", "Learn Vim Motions", "Buy a bread"];
    let done = Vec::<String>::new();
    let mut todo_curr: usize = 0;

    while !quit {
       for (idx, todo) in todos.iter().enumerate() {
           let pair = {
                if todo_curr == idx {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                }
           };

           attron(COLOR_PAIR(pair));
           mv(idx as i32, 1);
           addstr(*todo);
           attroff(COLOR_PAIR(pair));
       }

       refresh();

       let key = getch();
       match key as u8 as char {
           'q' => quit = true,
           'w' => if todo_curr > 0 {
                todo_curr -= 1;
           },
           's' => {
                todo_curr = min(todo_curr + 1, todos.len() - 1);
           },
            _ => {}
       }
    }
    endwin();
}
