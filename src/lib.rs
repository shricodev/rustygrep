use std::env;
use std::error::Error;
use std::fs;

pub struct InitialConfig<'a> {
    pub search_string: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}

impl<'a> InitialConfig<'a> {
    pub fn build(args: &[String]) -> Result<InitialConfig, &str> {
        // ["executable_path", "search_string", "file_name"] => This is the content for the args variable.
        if args.len() < 3 {
            return Err(
                "Not enough arguments. Please provide a search string and a file name. eg: 'rustygrep search_string file_name'",
            );
        }
        let search_string = &args[1];
        let file_path = &args[2];

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(InitialConfig {
            search_string,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(initial_config: InitialConfig) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(initial_config.file_path)?;

    let results = if initial_config.ignore_case {
        search_case_insensitive(&initial_config.search_string, &file_contents)
    } else {
        search(&initial_config.search_string, &file_contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "rust";
        let contents = "\
rust: safe, fast, productive.
Rust: This line should not be returned.";
        assert_eq!(
            vec!["rust: safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let contents = "\
Rust: safe, fast, productive.
rust: This line should also be returned.";
        assert_eq!(
            vec![
                "Rust: safe, fast, productive.",
                "rust: This line should also be returned."
            ],
            search_case_insensitive(query, contents)
        );
    }
}
