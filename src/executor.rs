use std::process::{self, ExitStatus};
use std::{os::unix::process::CommandExt, path::Path};

use crate::builtin::BuiltinKind;
use crate::{
    parser::{Command, ParsedInput, Word},
    shell,
    shell::Event,
    shell::ShellError,
};

pub enum Outcome {
    Event(Event),
    ExitStatus(ExitStatus),
    None,
}

pub fn dispatch(input: ParsedInput, state: &shell::State) -> Result<Outcome, ShellError> {
    let ParsedInput { source, program } = input;

    let Some(cmd) = program.into_iter().next() else {
        return Ok(Outcome::None);
    };

    match cmd {
        Command::Simple { argv } => {
            let command_name = argv[0].as_str(&source);
            match BuiltinKind::from_name(command_name) {
                Some(builtin) => builtin.execute(&source, argv, state),
                None => match state.path_vec.get_cmd_in_path(command_name) {
                    Some(cmd_path) => execute(&source, &argv, &cmd_path),
                    None => Ok(cmd_not_found(command_name)),
                },
            }
        }
    }
}

fn execute(source: &str, argv: &[Word], path: &Path) -> Result<Outcome, ShellError> {
    Ok(Outcome::ExitStatus(
        process::Command::new(path)
            .arg0(argv[0].as_str(source))
            .args(argv.iter().skip(1).map(|word| word.as_str(source)))
            .status()?,
    ))
}

fn cmd_not_found(command_name: &str) -> Outcome {
    println!("{command_name}: command not found");
    Outcome::None
}
