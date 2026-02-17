use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rust-fuzzy-clock")]
#[command(about = "A fuzzy clock that translates time into natural language", long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "english")]
    #[arg(help = "Language for time translation (english)")]
    pub language: String,
    
    #[arg(short, long, default_value = "exact")]
    #[arg(help = "Level of fuzzyness (exact)")]
    pub fuzzyness: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
