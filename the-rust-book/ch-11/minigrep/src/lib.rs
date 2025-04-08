use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(config.query.as_str(), contents.as_str())
    } else {
        search(config.query.as_str(), contents.as_str())
    };

    // search(config.query, config.contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String], ignore_case: bool) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            let is_match = line.contains(query);

            if is_match {
                println!("{line}");
            }

            is_match
        })
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            let is_match = line.to_lowercase().contains(query.to_lowercase().as_str());

            if is_match {
                println!("{line}");
            }

            is_match
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_one_result() {
        let query = "duct";
        let contents = vec!["Rust:", "safe, fast, productive.", "Pick three."].join("\n");

        assert_eq!(vec!["safe, fast, productive."], search(query, &contents));
    }

    #[test]
    fn should_return_no_result() {
        let query = "notintext";
        let contents = include_str!("../poem.txt");

        let result: Vec<&str> = vec![];
        assert_eq!(result, search(query, contents));
    }

    #[test]
    fn should_return_one_result_for_case_insensitive() {
        let query = "NAME";
        let contents = include_str!("../poem.txt");

        assert_eq!(
            vec!["To tell your name the livelong day"],
            search_case_insensitive(query, contents)
        )
    }

    #[test]
    fn should_return_no_result_for_case_insensitive() {
        let query = "notintext";
        let contents = include_str!("../poem.txt");

        let result: Vec<&str> = vec![];
        assert_eq!(result, search_case_insensitive(query, contents))
    }
}
