//================================================================================
//  Imports
//================================================================================

use std::fs;

mod scanner;

//================================================================================
//  Standalone Functions
//================================================================================

fn main() -> eyre::Result<()> {
    let file_path: String = String::from("./tests/hello.cl");
    let source: String = read_file(file_path)?;

    scanner::scan_tokens(source);

    Ok(())
}

fn read_file(path: String) -> Result<String, std::io::Error> {
    Ok(fs::read_to_string(path)?)    
}