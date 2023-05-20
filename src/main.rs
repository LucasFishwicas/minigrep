use std::env; // allows us to access the args iterator
use std::fs; // allows us to us the file system
use std::process; // allows us to manipulate the running process 
use std::error::Error; // allows us to access the Error trait object

fn main() {
    // store arguments in args - those given when 'cargo run' is called
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    
    // construct a Config instance or display an error message and the program
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}",err);
        process::exit(1);
    });
    
    // display a helpful message to the user
    println!("Searching for {}",config.query);
    println!("In file {}",config.file_path);

    // execute run, which reads and displays the given files contents
    // or display an error message and end the program
    if let Err(e) = run(config) {
        println!("Application error: {}",e);
        process::exit(1);
    }
        
}


// reads and displays the given files contents returns an error
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{}",contents);

    Ok(())
}


// Config struct to hold the query content collected from args
struct Config {
    query: String,
    file_path: String
}


// Config methods
impl Config {
    // constructor which will return an error if there are less than 3 args
    fn build(args: &[String]) -> Result<Config, &'static str> { 
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone(); 
        let file_path = args[2].clone();

        Ok(Config {query,file_path})
    }
}
