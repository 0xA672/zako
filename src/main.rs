use std::env;
use std::io::{BufRead, BufReader, Error, Write};
use std::process::Command;
use walkdir::WalkDir;
use std::path::PathBuf;


fn pwd() -> std::io::Result<PathBuf> {
    env::current_dir()
}

fn zig(){

}

fn scandir() -> std::io::Result<Vec<PathBuf>> {
    let cwd: PathBuf = pwd()?;
    let mut pathofzigzon: Vec<_> = Vec::<PathBuf>::new();
    loop {
       let potential: PathBuf = cwd.join("build.zig.zon");
       if potential.exists() {
        pathofzigzon.push(potential);
       }
    }
}

fn scanzon() {
    let r: Result<Vec<PathBuf>, Error> = scandir();
    match r {
        Ok(path) => {
        for i in path {
           
        }
      }
      Err(e) => {
        eprintln!("{}!",e)
      }
    }

    let mut output = File::create()?;
}

fn main(){

}