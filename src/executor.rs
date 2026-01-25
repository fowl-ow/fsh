use std::process;
use std::{os::unix::process::CommandExt, path::Path};

use crate::{
    builtin,
    parser::{Command, ParsedInput, Word},
    path::PathVec,
};

pub fn dispatch(input: ParsedInput, path_vec: &PathVec) -> Result<(), std::io::Error> {
    let ParsedInput { source, program } = input;

    for cmd in program {
        match cmd {
            Command::Simple { argv } => {
                let command_name = argv[0].as_str(&source);
                match command_name {
                    "exit" => process::exit(0),
                    "echo" => builtin::echo(&source, argv),
                    "type" => builtin::type_cmd(&source, argv, path_vec),
                    non_builtin => match path_vec.get_cmd_in_path(non_builtin) {
                        Some(cmd_path) => execute(&source, &cmd_path, argv),
                        None => cmd_not_found(command_name),
                    },
                }
            }
        }
    }

    Ok(())
}

fn execute(source: &str, path: &Path, argv: Vec<Word>) {
    match process::Command::new(path)
        .arg0(argv[0].as_str(source))
        .args(argv.iter().skip(1).map(|word| word.as_str(source)))
        .status()
    {
        Ok(_) => {}
        Err(e) => {
            println!("{e}")
        }
    }
}

fn cmd_not_found(command_name: &str) {
    println!("{}: command not found", command_name);
}
