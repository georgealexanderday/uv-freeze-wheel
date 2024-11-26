use std::path::PathBuf;

use clap::Parser;  

#[derive(Debug, Parser)]  
#[command(about = "Find all files containing a given name.")]  
pub struct Arguments {
    /// Name to find.  // (4)!
    #[arg(short, long)]  
    pub name: String, 
    /// Path to to check.
    #[arg(default_value = ".")] 
    pub path: PathBuf, 
}

fn main() {
    let args = Arguments::parse();  
    println!("{:?}", args);
}
