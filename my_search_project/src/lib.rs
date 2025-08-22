use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Self, String> {
        if args.len() != 3 {
            return Err("query and file_path are required".to_string());
        }
        let mut ignore_case = false;
        match std::env::var("IGNORE_CASE") {
            Ok(_) => ignore_case = true,
            _ => ignore_case = false,
        }
        Ok(Config {
            // ignora args[0] = nome do executável
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), std::io::Error> {
    let file = File::open(config.file_path)?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line_copy = line?;
        if config.ignore_case {
            let query = config.query.to_uppercase();
            let line_uc = line_copy.to_uppercase();
            if line_uc.contains(query.as_str()) {
                println!("{}: {}", i, line_copy);
            }
        } else if line_copy.contains(config.query.as_str()) {
            println!("{}: {}", i, line_copy);
        }
    }
    Ok(())
}
