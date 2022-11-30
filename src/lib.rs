//! # andersonpgrep
//!
//! `andersonpgrep` is a cmd tool to find text in a lines of text
//! how to run:
//! `andersonpgrep <query> <path to file>`
//! to run in ignorecase set the enviroment variable to any value
//! for example: in unix
//! `IGNORE_CASE=1 andersonpgrep hello file.txt`
//! or using powershell in windows
//! `$env:IGNORE_CASE=1` \
//! the run \
//! `andersonpgrep hello file.txt`

use std::error::Error;
use std::{env, fs};
pub mod closures_and_iterators;

/// Driver code
/// # Examples
/// ```
/// use std::env;
/// use andersonpgrep::Config;
/// let kwargs: Vec<String> = env::args().collect();
/// let config = Config::build(env::args());
///
/// ```
///
/// # Panics
/// will panic if the argument count is less than 2
/// ```
/// use std::env;
/// use andersonpgrep::Config;
/// let config = Config::build(env::args());
///
/// ```

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?
        //.expect("Should have been able to read the file")
        ;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }

    Ok(())
}

#[derive(Debug)]
/// the configuration struct
/// to build one, consumes iterator
/// #Examples
/// ```
/// ```
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    /// builds a Config object
    /// # Errors
    /// ```
    /// use std::env;
    /// use andersonpgrep::Config;
    /// // will return error if args is less than 3
    /// let config = Config::build(env::args());
    /// assert_eq!(config, Config{
    /// query: String::from("query"),
    /// file_path: String::from("file.txt"), ignore_case: false
    /// }, "please run cargo test -- query file.txt")
    ///
    /// ```
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // ignore the first argument since it the path to this binary execution
                     // if args.len() < 3 {
                     //     return Err("not enough arguments")
                     // }
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
/// searches all lines containing query, in a case-sensitive fashion
/// # Examples
/// ```
/// use andersonpgrep::search;
/// let query = "ask";
/// let contents = "\
/// If you Ask me I will say
/// please ask her
/// about tha issue
/// ";
/// let result = search(query, contents);
///assert_eq!(vec!["please ask her"], result);
/// ```
/// # Panics
/// ```
///
/// ```
///
/// # Errors
/// ```
///
/// ```
///
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
/// searches all lines containing query, in a case-insensitive fashion
///
/// # Examples
/// ```
/// use andersonpgrep::search_case_insensitive;
/// let query = "ask";
/// let contents = "\
/// If you Ask me I will say
/// please ask her
/// about tha issue
/// ";
/// let result = search_case_insensitive(query, contents);
///assert_eq!(vec!["If you Ask me I will say","please ask her"], result);
/// ```
/// # Panics
/// ```
///
/// ```
///
/// # Errors
/// ```
///
/// ```
///
///
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
