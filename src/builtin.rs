use crate::{
    executor::Outcome,
    parser::Word,
    path::PathVec,
    shell::{Event, ShellError, State},
};
use std::path::Path;

pub enum BuiltinKind {
    Echo,
    Exit,
    Type,
    Pwd,
}

impl BuiltinKind {
    pub fn from_name(name: &str) -> Option<BuiltinKind> {
        match name {
            "echo" => Some(BuiltinKind::Echo),
            "exit" => Some(BuiltinKind::Exit),
            "type" => Some(BuiltinKind::Type),
            "pwd" => Some(BuiltinKind::Pwd),
            _ => None,
        }
    }

    pub fn execute(
        &self,
        source: &str,
        argv: Vec<Word>,
        state: &State,
    ) -> Result<Outcome, ShellError> {
        match self {
            Self::Echo => {
                echo(source, argv)?;
                Ok(Outcome::None)
            }
            Self::Exit => Ok(Outcome::Event(Event::Exit)),
            Self::Type => {
                type_cmd(source, argv, &state.path_vec)?;
                Ok(Outcome::None)
            }
            Self::Pwd => {
                print_working_directory(&state.working_dir)?;
                Ok(Outcome::None)
            }
        }
    }
}

fn echo(source: &str, argv: Vec<Word>) -> Result<(), ShellError> {
    if argv.len() > 1 {
        let s = argv[1..]
            .iter()
            .map(|word| word.as_str(source))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", s);
    }
    Ok(())
}

fn type_cmd(source: &str, argv: Vec<Word>, path_vec: &PathVec) -> Result<(), ShellError> {
    if let Some(word) = argv.get(1) {
        let command_name = word.as_str(source);
        match BuiltinKind::from_name(command_name) {
            Some(_) => {
                println!("{command_name} is a shell builtin");
            }
            None => match path_vec.get_cmd_in_path(command_name) {
                Some(path) => println!("{command_name} is {}", path.display()),
                None => println!("{command_name}: not found"),
            },
        }
    }
    Ok(())
}

fn print_working_directory(path: &Path) -> Result<(), ShellError> {
    println!("{}", path.display());
    Ok(())
}
