use crate::shell::ShellError;

mod builtin;
mod executor;
mod parser;
mod path;
mod scanner;
mod shell;

fn main() -> Result<(), ShellError> {
    crate::shell::run()?;
    Ok(())
}
