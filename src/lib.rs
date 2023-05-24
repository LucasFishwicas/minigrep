use std::fs; // allows us to us the file system
use std::error::Error; // allows us to access the Error trait object
use std::env;


// reads and displays the given files contents returns an error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
      search_case_insensitive(&config.query, &contents)  
    } else {
        search(&config.query, &contents)
    };

    for line in results{
        println!("{}",line);
    }

    Ok(())
}


// Config struct to hold the query content collected from args
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}


// Config methods
impl Config {
    // constructor takes any type that implements the Iterator trait and returns
    // String values;
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> { 
        args.next(); // skip the name of the program
        
        // assign the second value to query or return Err()
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        // assign the third value to file_path or return Err()
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}



// Iterate through lines in contents and filter for lines that contain the query
// string; use collect() to create a new Vec with the results 
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents  // the given string slice
        .lines() // create an Iterator for the lines in contents
        .filter(|line| line.contains(query)) // filter lines with closure
        .collect()  // store filtered results into new Vec and return it
}



// Iterate through lines in contents and filter for lines that contain the query
// string (case insensitive); use collect() to create a new Vec with the results
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    // set query to lowercase
    let query = query.to_lowercase();
  
    contents  // the given string slice
        .lines() // create an Iterator for the lines in contents
        .filter(|line| line.to_lowercase().contains(&query)) // filter lines
        .collect() // store filtered results into new Vec and return it
}







// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query,contents)
        );
    }
}
