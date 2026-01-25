use crate::{parser::Word, path::PathVec};

pub fn echo(source: &str, argv: Vec<Word>) {
    if argv.len() > 1 {
        let s = argv[1..]
            .iter()
            .map(|word| word.as_str(source))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", s);
    }
}

pub fn type_cmd(source: &str, argv: Vec<Word>, path_vec: &PathVec) {
    match argv.get(1) {
        None => {}
        Some(word) => {
            let span = word.span;
            let command_name = &source[span.start..span.end];
            match command_name {
                "exit" | "echo" | "type" => {
                    println!("{command_name} is a shell builtin")
                }
                _ => match path_vec.get_cmd_in_path(command_name) {
                    Some(path) => println!("{command_name} is {}", path.display()),
                    None => println!("{command_name}: not found"),
                },
            }
        }
    }
}
