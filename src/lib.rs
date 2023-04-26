use std::io::{BufReader, BufRead};
use clap::Parser;

#[derive(Parser)]
#[command(name = "resplit")]
#[command(version = "0.0.1")]
#[command(author = "Alfredo Deza")]
#[command(about = "Split strings by one or more delimeters and return a field, like cut")]
pub struct Cli {
    #[arg(short('f'))]
    field: usize,
    #[arg(short('d'))]
    delimeter: String,
    #[arg(long)]
    debug: bool,
}

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read input line");
    line.trim().to_string()
}


pub fn split(s: String, cli: &Cli) -> String {
    let parts: Vec<&str> = s.split(&cli.delimeter).collect();
    if cli.debug {
        println!("Parts: {:?}", parts);
        println!("Indexes available starting at 0: {}", parts.len());
    }
    parts.get(cli.field).unwrap_or(&"").to_string()
}