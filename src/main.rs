mod builtin;
mod executor;
mod parser;
mod path;
mod scanner;
mod shell;

#[allow(unused_imports)]
fn main() -> Result<(), std::io::Error> {
    crate::shell::run()
}
