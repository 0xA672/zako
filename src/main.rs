use std::env;
use std::io::{BufRead, BufReader, Error, Write};
use std::process::Command;
use walkdir::WalkDir;
use std::path::PathBuf;


fn pwd() -> std::io::Result<PathBuf> {
    env::current_dir()
}

fn scandir() -> std::io::Result<Vec<PathBuf>> {
    let cwd = pwd()?;
    let mut pathofzigzon = Vec::new();
    for i in WalkDir::new(cwd) {
        let i = i.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;///turn walkdir::Error into ErrorKind::Other
        if i.file_name() == "build.zig.zon" {
            pathofzigzon.push(i.into_path());
        }
    }
    Ok(pathofzigzon)
}

fn scanzon() {
    let path = scandir();
}

fn main(){

}