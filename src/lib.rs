use std::fs; // allows us to us the file system
use std::error::Error; // allows us to access the Error trait object



// reads and displays the given files contents returns an error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{}",contents);

    Ok(())
}


// Config struct to hold the query content collected from args
pub struct Config {
    pub query: String,
    pub file_path: String
}


// Config methods
impl Config {
    // constructor which will return an error if there are less than 3 args
    pub fn build(args: &[String]) -> Result<Config, &'static str> { 
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone(); 
        let file_path = args[2].clone();

        Ok(Config {query,file_path})
    }
}
