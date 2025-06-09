#![allow(warnings)]

use std::{fs, path::PathBuf};
use clap::Parser;
use eyre::{Result, Context};
use crate::ast::{Class, Feature, VarDecl, ArgDecl, Expr, TypedExpr};

mod ast;
mod parsing;
mod semantic;
mod cool;


/// Command-line options
#[derive(Parser)]
#[command(name = "cool-rs", version, about = "A COOL language compiler written in Rust")]
struct Cli {
    /// Path to the input COOL source file
    #[arg(short, long, value_name = "FILE")]
    file: PathBuf,
}

/// Read the entire file into a String, with context on errors
fn read_file(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).wrap_err_with(|| format!("Failed to read source file: {:?}", path))
}


/// Returns a Vec<Class> containing Object, IO, String, Int and Bool,
/// each with dummy TypedExpr bodies (line = 0).
fn builtin_classes() -> Vec<Class> {
    let mut result = Vec::new();

    // 1) Object
    result.push(Class {
        name: "Object".to_string(),
        inherits: None,
        feature_list: vec![
            // abort(): Object { abort }
            Feature::Method(
                "abort".to_string(),
                Vec::new(),
                "Object".to_string(),
                // TypedExpr::new(expr, line)
                TypedExpr::new(Expr::Identifier("abort".to_string()), 0),
            ),
            // type_name(): String { "Object" }
            Feature::Method(
                "type_name".to_string(),
                Vec::new(),
                "String".to_string(),
                TypedExpr::new(Expr::Str("Object".to_string()), 0),
            ),
        ],
    });

    // 2) IO inherits Object
    result.push(Class {
        name: "IO".to_string(),
        inherits: Some("Object".to_string()),
        feature_list: vec![
            // out_string(str: String): IO { self }
            Feature::Method(
                "out_string".to_string(),
                vec![ArgDecl::new("str".to_string(), "String".to_string())],
                "IO".to_string(),
                TypedExpr::new(Expr::Identifier("self".to_string()), 0),
            ),
            // out_int(i: Int): IO { self }
            Feature::Method(
                "out_int".to_string(),
                vec![ArgDecl::new("i".to_string(), "Int".to_string())],
                "IO".to_string(),
                TypedExpr::new(Expr::Identifier("self".to_string()), 0),
            ),
            // in_string(): String { "" }
            Feature::Method(
                "in_string".to_string(),
                Vec::new(),
                "String".to_string(),
                TypedExpr::new(Expr::Str("".to_string()), 0),
            ),
            // in_int(): Int { 0 }
            Feature::Method(
                "in_int".to_string(),
                Vec::new(),
                "Int".to_string(),
                TypedExpr::new(Expr::Int(0), 0),
            ),
        ],
    });

    // 3) String inherits Object
    result.push(Class {
        name: "String".to_string(),
        inherits: Some("Object".to_string()),
        feature_list: vec![
            // length(): Int { 0 }
            Feature::Method(
                "length".to_string(),
                Vec::new(),
                "Int".to_string(),
                TypedExpr::new(Expr::Int(0), 0),
            ),
            // concat(s: String): String { self }
            Feature::Method(
                "concat".to_string(),
                vec![ArgDecl::new("s".to_string(), "String".to_string())],
                "String".to_string(),
                TypedExpr::new(Expr::Identifier("self".to_string()), 0),
            ),
            // substr(i: Int, l: Int): String { self }
            Feature::Method(
                "substr".to_string(),
                vec![
                    ArgDecl::new("i".to_string(), "Int".to_string()),
                    ArgDecl::new("l".to_string(), "Int".to_string()),
                ],
                "String".to_string(),
                TypedExpr::new(Expr::Identifier("self".to_string()), 0),
            ),
        ],
    });

    // 4) Int inherits Object (no methods)
    result.push(Class {
        name: "Int".to_string(),
        inherits: Some("Object".to_string()),
        feature_list: Vec::new(),
    });

    // 5) Bool inherits Object (no methods)
    result.push(Class {
        name: "Bool".to_string(),
        inherits: Some("Object".to_string()),
        feature_list: Vec::new(),
    });

    result
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();
    let source = read_file(&cli.file)?;

    // Lexing
    let mut scanner = parsing::scanner::Scanner::new(&source);
    let tokens = scanner.scan_tokens().unwrap();
    let token_iter = tokens.into_iter().map(|(tok, loc)| {
        Ok((loc.line, tok, loc.line))
    });

    // Parsing
    let program = cool::ProgramTyParser::new()
        .parse(token_iter)
        .wrap_err("Parsing failed")?;

    let mut ast: Vec<ast::Class> = program.classes;

    let mut builtins = builtin_classes();
    let existing: std::collections::HashSet<_> =
        ast.iter().map(|c| c.name.clone()).collect();
    builtins.retain(|c| !existing.contains(&c.name));
    
    builtins.append(&mut ast);
    let ast = builtins;

    // Display the parsed AST
    println!("Parsed AST ({} classes):", ast.len());
    for class in &ast {
        println!("{:#?}", class);
    }

    // Semantic Phases
    let mut ec = semantic::collector::ErrorCollector::default();

    // Inheritance checks
    semantic::analyzer::check_inheritance(&ast, &mut ec);
    if ec.has_errors() {
        ec.report_all();
        std::process::exit(1);
    }

    // Attribute/Method symbol checks
    semantic::symbols::check_class_features(&ast, &mut ec);
    if ec.has_errors() {
        ec.report_all();
        std::process::exit(1);
    }

    // Expression/type checks
    semantic::type_checker::check_expressions(&ast, &mut ec);
    if ec.has_errors() {
        ec.report_all();
        std::process::exit(1);
    }

    println!("Semantic checks passed without errors.");
    Ok(())
}
