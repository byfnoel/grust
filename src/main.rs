/* A grep command line tool written in rust */

#![allow(unused)]
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct CliArgs {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = CliArgs::parse();
    let content = fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
