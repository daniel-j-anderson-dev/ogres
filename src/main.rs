fn main() {
    println!("Hello, world!");
}

fn get_input(prompt: &str) -> Result<String, std::io::Error> {
    use std::io::{stdin, stdout, Write};

    stdout().write(prompt.as_bytes())?;
    stdout().flush()?;
    
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    return Ok(input.trim().to_string());
}

/*

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file_path> <regex>", args[0]);
        std::process::exit(1);
    }

    // Extract file path and regex pattern
    let file_path = &args[1];
    let regex_pattern = &args[2];

    // Read file line by line and search for the pattern
    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        // Create a regex object
        let regex = Regex::new(regex_pattern).expect("Invalid regex pattern");

        for (line_number, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                if regex.is_match(&line) {
                    // Match found, print the result
                    println!(
                        "Found '{}' in '{}' @ line {}, word {}",
                        regex_pattern,
                        file_path,
                        line_number + 1,
                        find_word_index(&line, regex_pattern)
                    );
                }
            }
        }
    } else {
        eprintln!("Error reading file: {}", file_path);
        std::process::exit(1);
    }
}

// Helper function to find the word index in a line
fn find_word_index(line: &str, word: &str) -> usize {
    line.split_whitespace().position(|w| w == word).unwrap_or(0) + 1
}


*/