use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::command::Args;

pub fn new(args: &Args) -> Result<()> {
    let path = PathBuf::from(&args.dir);
    
    if path.exists() {
        if args.force {
            std::fs::remove_dir_all(&path)
                .with_context(|| format!("Failed to remove existing directory: {}", args.dir))?;
            println!("Removed existing directory: {}", args.dir);
        } else {
            return Err(anyhow!("Project already exists. Use --force to overwrite"));
        }
    }
    
    std::fs::create_dir_all(&path)
        .with_context(|| format!("Failed to create directory: {}", args.dir))?;
    
    println!("Created new project at: {}", args.dir);
    Ok(())
}