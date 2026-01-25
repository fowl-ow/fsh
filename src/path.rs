use nix::unistd;
use std::env::{self};
use std::fs::{self};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

pub struct PathVec(Vec<PathBuf>);

impl PathVec {
    pub fn new() -> Self {
        Self(get_path_env())
    }

    pub fn get_cmd_in_path(self: &PathVec, key: &str) -> Option<PathBuf> {
        for path in self.iter() {
            let file_path = path.join(key);
            if is_exec_cmd(&file_path) {
                return Some(file_path);
            }
        }
        None
    }
}

fn is_exec_cmd(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path)
        && metadata.is_file()
        && is_exec(path)
    {
        true
    } else {
        false
    }
}

// check if file or / dir is executable
fn is_exec(path: &Path) -> bool {
    match unistd::access(path, unistd::AccessFlags::X_OK) {
        Ok(()) => true,
        Err(_) => false,
    }
}

fn get_path_env() -> Vec<PathBuf> {
    let key = "PATH";
    let mut p: Vec<PathBuf> = Vec::new();
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                p.push(path);
            }
            p
        }
        None => {
            println!("{key} is not defined in the environment.");
            p
        }
    }
}

impl Deref for PathVec {
    type Target = Vec<PathBuf>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PathVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
