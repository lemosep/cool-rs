use std::{fs, path::PathBuf};
use clap::Parser;
use eyre::{Result, Context};

mod ast;
mod parsing;
mod cool;

/// Command-line options
#[derive(Parser)]
#[command(name = "cool-rs", version, about = "A COOL language parser")] 
struct Cli {
    /// Path to the input COOL source file
    #[arg(short, long, value_name = "FILE")]
    file: PathBuf,
}

/// Read the entire file into a String, with context on errors
fn read_file(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path)
        .wrap_err_with(|| format!("Failed to read source file: {:?}", path))
}

fn main() -> eyre::Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Load source
    let source = read_file(&cli.file)?;

    // Lexing
    let mut scanner = parsing::scanner::Scanner::new(&source);
    let tokens = scanner
        .scan_tokens().unwrap();

    let token_iter = tokens
        .into_iter()
        .map(|(tok, loc)| {
        Ok((loc, tok, loc))
        });

    let ast: Vec<ast::Class> = cool::ProgramParser::new()
        .parse(token_iter)
        .wrap_err("Parsing failed")?;

    // Display the parsed AST
    println!("Parsed AST ({} classes):", ast.len());
    for class in ast {
        println!("{:#?}", class);
    }

    Ok(())
}
