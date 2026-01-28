use std::{
    io::{Error, Write, stdin, stdout},
    path::PathBuf,
};

use crate::{
    executor::{self, Outcome},
    parser,
    path::PathVec,
    scanner,
};

pub struct State {
    pub path_vec: PathVec,
    pub working_dir: PathBuf,
}

pub enum Event {
    Exit,
}

#[derive(Debug)]
pub enum ShellError {
    #[allow(dead_code)]
    IO(Error),
}

impl From<Error> for ShellError {
    fn from(e: Error) -> Self {
        ShellError::IO(e)
    }
}

pub fn run() -> Result<(), ShellError> {
    let state: State = State {
        path_vec: PathVec::new(),
        working_dir: get_working_dir()?,
    };

    loop {
        show_prompt();
        flush();
        let input = get_input()?;
        let scanned_input = scanner::scan(input);
        let parsed_input = parser::parse(scanned_input);
        match executor::dispatch(parsed_input, &state)? {
            Outcome::Event(event) => match event {
                Event::Exit => return Ok(()),
            },
            Outcome::ExitStatus(_status) => {}
            Outcome::None => {}
        }
        flush();
    }
}

fn show_prompt() {
    print!("$ ");
}

fn flush() {
    stdout().flush().unwrap();
}

fn get_input() -> Result<String, Error> {
    let mut input = String::new();

    stdin().read_line(&mut input)?;
    Ok(input.to_string())
}

fn get_working_dir() -> Result<PathBuf, Error> {
    if let Some(dir) = std::env::var_os("PWD") {
        Ok(PathBuf::from(dir))
    } else {
        Ok(std::env::current_dir()?)
    }
}
