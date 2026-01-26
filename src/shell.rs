use std::io::{Write, stdin, stdout};

use crate::{executor, parser, path::PathVec, scanner};

struct State {
    path_vec: PathVec,
}

pub fn run() -> Result<(), std::io::Error> {
    let state: State = State {
        path_vec: PathVec::new(),
    };

    loop {
        show_prompt();
        flush();
        let input = get_input();
        let scanned_input = scanner::scan(input);
        let parsed_input = parser::parse(scanned_input);
        let _ = executor::dispatch(parsed_input, &state.path_vec);
        flush();
    }
}

fn show_prompt() {
    print!("$ ");
}

fn flush() {
    stdout().flush().unwrap();
}

fn get_input() -> String {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");
    input.to_string()
}
