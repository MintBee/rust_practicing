use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        Ok(Config { query, filename })
    }
}

pub fn print_file_text(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, base_string: &'a str) -> Option<Vec<&'a str>> {
    search_with_filter(&base_string, |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(query: &str, base_string: &'a str) -> Option<Vec<&'a str>> {
    let query = query.to_lowercase();
    search_with_filter(base_string, |line| line.to_lowercase().contains(&query))
}

pub fn search_with_filter<'a, F>(base_string: &'a str, mut f: F) -> Option<Vec<&'a str>>
    where
        F: FnMut(&str) -> bool,
{
    let result: Vec<&'a str> = base_string.lines().filter(|line| f(line)).collect();
    return if !result.is_empty() {
        Some(result)
    } else {
        None
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let content = "\
This is Rust.
effective, good, productive
language.";
        assert_eq!(
            vec!["effective, good, productive"],
            search_case_sensitive(query, content).unwrap()
        );
    }
}
