use std::{env, fs};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(file_path) => file_path,
        None => return Err("Missing file path.".into()),
    };
    let regex = match args.get(2) {
        Some(regex_pattern) => Regex::new(regex_pattern)?,
        None => return Err("Missing regex pattern.".into()),
    };

    let file_data = fs::read_to_string(file_path)?;

    for (line_number, line) in file_data.lines().enumerate() {
        
        for regex_match in regex.find_iter(line) {
            let word_index = find_word_index(line, regex_match.start());

            println!(
                "Found \"{}\" in \"{}\" @ line \"{}\", word \"{}\"",
                regex,
                file_path,
                line_number + 1,
                word_index,
            );
        }
        
    }

    return Ok(());
}

/*
find_word_index
    in
        line: &str
        word: &str
    out
        usize
    do
        line
            .split_whitespace()
            .position(|w| w == word)
            .unwrap_or(0) + 1
*/

fn find_word_index(line: &str, start_of_word: usize) -> usize {

    let mut word_index = 0;
    
    for (character_index, character) in line.char_indices() {
        if character.is_whitespace() {
            word_index += 1;
        }
        if character_index == start_of_word {
            break;
        }
    }

    return word_index;
}