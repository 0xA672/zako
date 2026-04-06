use clap::Parser;

#[derive(Parser, Clone)]
pub struct Args {
    #[arg(short, long)]
    pub dir: String,
    pub force: bool,
    pub lib: bool,
}