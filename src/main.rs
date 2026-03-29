pub(crate) mod lexer;

use std::env;
use std::io::{BufRead, BufReader, Error, Write};
use std::process::Command;
use walkdir::WalkDir;
use std::path::PathBuf;
use std::fs::File;


    

fn pwd() -> std::io::Result<PathBuf> {
    env::current_dir()
}

fn zig(){

}

fn scandir() -> std::io::Result<Vec<PathBuf>> {
    let mut cwd: PathBuf = pwd()?;
    let mut pathofzigzon: Vec<PathBuf> = Vec::new();
    let r = loop {
       let potential: PathBuf = cwd.join("build.zig.zon");
       if potential.exists() {
        pathofzigzon.push(potential);
       }
       if let Some(cddotdot) =  cwd.parent() {
           cwd = cddotdot.to_owned();
       }else { break pathofzigzon }
    };
    Ok(r)
}

fn scanzon()  {
    let r: Result<Vec<PathBuf>, Error> = scandir();
    match r {
        Ok(path) => {
        for i in path {
           let mut output = File::open(i);

        }
      }
      Err(e) => {
        eprintln!("{}!",e)
      }
    }

}

fn main(){

}