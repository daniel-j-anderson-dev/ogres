fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    return Ok(());
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
main
    args = collect args from environment
    file_path       = args[1]
    regex_pattern   = args[2]

    for each line in file
        if line matches regex
            print 
                "Found '{}' in '{}' @ line {}, word {}",
                regex_pattern,
                file_path,
                line_number + 1,
                find_word_index(line, regex_pattern)
*/