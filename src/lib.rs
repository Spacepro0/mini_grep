use std::collections::HashMap;
use std::error::Error;
use std::fs;

// if it does not fail return nothing else return an error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut content_list: HashMap<String, String> = HashMap::new();
    for file in &config.file_list {
        content_list.insert(file.clone(), fs::read_to_string(file.clone())?);
    }

    let results = if !config.case_sensitive {
        search_case_insensitive(&config.query, &content_list)
    } else {
        search(&config.query, &content_list)
    };

    // print the output to usets
    for line in results {
        let mut p = true;

        for file_name in &config.file_list {
            if line == file_name.as_str() {
                p = false;
                println!("\n{}:", line);
            }
        }
        if p {
            println!("    {}", line);
        }
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_list: Vec<String>,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &mut Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            if args.len() == 2 {
                if args[1].to_lowercase() == "help" {
                    help();
                }
            }
            return Err("Not enough arguments");
        }

        // use an environment variable to determin the case sensitivity if the variable is set

        let mut case_sensitive = false;
        if args[2] == String::from("1") {
            case_sensitive = true;
            args.remove(2);
        }

        // use the first argument as the string to search for
        // uses clone to duplicate the variable without taking ownership
        let query = args[1].clone();

        let mut file_list: Vec<String> = Vec::new();

        for arg in args.clone() {
            if !(arg == args[0] || arg == args[1]) {
                file_list.push(arg.clone());
            }
        }

        Ok(Config {
            query,
            file_list,
            case_sensitive,
        })
    }
}

// search through the content and find the matching lines
pub fn search<'a>(query: &str, contents_list: &'a HashMap<String, String>) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    // loop throug the hasmap and print the values
    for file_content in contents_list {
        results.push(file_content.0);
        for line in file_content.1.lines() {
            if line.contains(&query) {
                results.push(line);
            }
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents_list: &'a HashMap<String, String>,
) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    // loop throug the hasmap and print the values
    for file_content in contents_list {
        results.push(file_content.0);
        for line in file_content.1.lines() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                results.push(line);
            }
        }
    }

    results
}

pub fn help() {
    println!("Help menu:");
    println!(
        "Mini Grep will search for a string in a list of files and return all lines that have the string."
    );
    println!(
        "To use type 'minigrep <string-to-search> <*optional*-case_sensitive> <file-to-search-in>'"
    );
    println!("
    You do not need to provide a value for the case-sensitive argument. By default the search is not case sensitive. \nAdd the number 1 after the search term to make the query cas sensitive."
    );
    println!("Usage example: 'mini_grep hello poem.txt, poem2.txt'");
    println!("Make sure that the file passed in has valid UTF-8 characters and exist in the current directory.");
    println!("For help again, type 'mini-grep help'");
    std::process::exit(0);
}

// testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let mut contents: HashMap<String, String> = HashMap::new();
        contents.insert(
            String::from("file.txt"),
            String::from(
                "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.",
            ),
        );

        assert_eq!(
            vec!["file.txt", "safe, fast, productive."],
            search(query, &contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let mut contents: HashMap<String, String> = HashMap::new();
        contents.insert(
            String::from("file.txt"),
            String::from(
                "\
Rust:
safe, fast, productive.
Pick three.",
            ),
        );

        assert_eq!(
            vec!["file.txt", "Rust:"],
            search_case_insensitive(query, &contents),
        );
    }
}
