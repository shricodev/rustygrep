use std::error::Error;
use std::fs;

pub struct InitialConfig<'a> {
    pub search_string: &'a str,
    pub file_path: &'a str,
}

impl<'a> InitialConfig<'a> {
    pub fn build(args: &[String]) -> Result<InitialConfig, &str> {
        // ["executable_path", "search_string", "file_name"] => This is the output for the args variable.
        if args.len() < 3 {
            return Err("Not enough arguments. Please provide a search string and a file name.");
        }
        let search_string = &args[1];
        let file_path = &args[2];
        Ok(InitialConfig {
            search_string,
            file_path,
        })
    }
}

pub fn run(initial_config: InitialConfig) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(initial_config.file_path)?;
    for line in search(&initial_config.search_string, &file_contents) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "rust";
        let contents = "\
        rust: safe, fast, productive.";
        assert_eq!(
            vec!["rust: safe, fast, productive."],
            search(query, contents)
        );
    }
}
