// use core::fmt;
// use std::path::PathBuf;
//
// #[derive(Debug)]
// pub enum Cmd {
//     Builtin(BuiltinCmd),
//     External {
//         name: String,
//         path: PathBuf,
//         args: Args,
//     },
// }
//
// #[derive(Debug)]
// pub enum BuiltinCmd {
//     Echo(Args),
//     Type(Args),
//     Exit,
// }
//
// impl std::fmt::Display for BuiltinCmd {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", format!("{:?}", self).to_lowercase())
//     }
// }
//
// pub type Args = Vec<String>;
