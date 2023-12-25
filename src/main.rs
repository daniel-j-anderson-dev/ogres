fn main() {
    println!("Hello, world!");
}

fn get_input(prompt: &str) -> Result<String, std::io::Error> {
    use std::io::{stdin, stdout, Write};

    stdout().write(prompt.as_bytes())?;
    
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    return Ok(input.trim().to_string());
}
