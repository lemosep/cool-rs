//================================================================================
//  Imports
//================================================================================

use std::{fs, path::PathBuf};
mod scanner;

use clap::Parser;

//================================================================================
//  CLI
//================================================================================

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name="FILE")]
    file: PathBuf
}

//================================================================================
//  Standalone Functions
//================================================================================

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    let file_path = if let Some(p) = cli.file.to_str() {
        p
    } else {
        panic!("Error: required argument missing: file")
    };

    let source: String = read_file(file_path.to_owned())?;

    scanner::scan_tokens(&source);

    Ok(())
}

fn read_file(path: String) -> Result<String, std::io::Error> {
    Ok(fs::read_to_string(path)?)    
}
