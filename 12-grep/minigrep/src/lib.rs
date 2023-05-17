use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        match args.len() {
            0 => Err(format!("These are not the program args")),
            1 => Err(format!("No argument was passed to {}", args[0].split('/').last().unwrap())),
            2 => Err(format!("No filename was passed to {}", args[0].split('/').last().unwrap())),
            _ => {
                let query = args[1].clone();
                let file_path = args[2].clone();
    
                Ok(Config { query, file_path })
            }
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
