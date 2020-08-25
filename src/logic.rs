use std::fs;
use core::result::Result::{Ok, Err};
use std::error::Error;
use std::env;


pub fn grep_logic(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let res = if config.case_sense {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in res {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sense: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sense = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sense })
    }
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = vec![];

    for line in contents.lines() {
        if line.contains(&query) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_res() {
        let query = "duct";
        let contents = "\n
        Rust: \n
        safe, fast, productive\n
        . pick all";

        assert_eq!(vec!["        safe, fast, productive"],
                   search_case_sensitive(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "rUsT";
        let contents = "\n
        Rust: \n
        safe, fast, productive\n
        . pick all";

        assert_eq!(vec!["        Rust: "],
                   search_case_insensitive(query, contents));
    }
}