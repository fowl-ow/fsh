use crate::{parser::Word, path::PathVec};
use std::process;

pub enum BuiltinKind {
    Echo,
    Exit,
    Type,
    // Pwd,
    // Cd
}

impl BuiltinKind {
    pub fn from_name(name: &str) -> Option<BuiltinKind> {
        match name {
            "echo" => Some(BuiltinKind::Echo),
            "exit" => Some(BuiltinKind::Exit),
            "type" => Some(BuiltinKind::Type),
            _ => None,
        }
    }

    pub fn execute(&self, source: &str, argv: Vec<Word>, path_vec: &PathVec) {
        match self {
            Self::Echo => echo(source, argv),
            Self::Exit => process::exit(0),
            Self::Type => {
                type_cmd(source, argv, path_vec);
            }
        }
    }
}

fn echo(source: &str, argv: Vec<Word>) {
    if argv.len() > 1 {
        let s = argv[1..]
            .iter()
            .map(|word| word.as_str(source))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", s);
    }
}

fn type_cmd(source: &str, argv: Vec<Word>, path_vec: &PathVec) {
    if let Some(word) = argv.get(1) {
        let command_name = word.as_str(source);
        match BuiltinKind::from_name(command_name) {
            Some(_) => {
                println!("{command_name} is a shell builtin")
            }
            None => match path_vec.get_cmd_in_path(command_name) {
                Some(path) => println!("{command_name} is {}", path.display()),
                None => println!("{command_name}: not found"),
            },
        }
    }
}
