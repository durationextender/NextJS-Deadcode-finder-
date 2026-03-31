mod errors;
mod file_scanner;
mod graph;
mod lexers;
mod parser;
use crate::graph::{build_graph, find_dead_exports};

use crate::file_scanner::scan_directories;
use crate::lexers::Lexer;
use std::io;
use std::path::Path;

use crate::parser::Parser as MyParser;

use clap::Parser as CliArgs;

#[derive(CliArgs, Debug)]
#[command(version, author, about, long_about = None)]
struct Args {
    /// The directory path you want to scan for dead code
    #[arg(short, long, default_value = "./src")]
    input: String,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    let path = Path::new(&args.input);

    println!("Scanning directory: {:?}", path);

    let file_paths = scan_directories(path)?;

    let graph = build_graph(file_paths);

    let dead_exports = find_dead_exports(&graph, path);

    println!("\n Results ");
    if dead_exports.is_empty() {
        println!("No dead exports found.");
    } else {
        println!("Found {} dead export(s):", dead_exports.len());
        for (file, name) in dead_exports {
            println!("   - [{}] in file: {}", name, file.display());
        }
    }

    Ok(())
}
