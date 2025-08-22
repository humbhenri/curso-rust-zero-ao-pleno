use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line_copy = line?.clone();
        if line_copy.is_empty() {
            break;
        }
        lines.push(line_copy);
    }
    lines.iter().rev().for_each(|line| {
        println!("{}", line);
    });
    Ok(())
}
