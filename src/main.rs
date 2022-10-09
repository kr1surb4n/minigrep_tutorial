use std::env;
use std::fs;
use std::error::Error;
use std::process;

fn main() {
    // get all command line args and load them into structure
    let args: Vec<String> = env::args().collect();
    // will panic on non-unicode sign
    // std::env::args_os -> unicode safe

    dbg!(&args);
    // args = [
    // "target/debug/minigrep",
    // "searchstring",
    // "example-filename.txt",
    // ]

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

#[test]
fn parse_this() {
    let args: Vec<String> = vec![String::from("0"), String::from("a"), String::from("b")];

    let config = Config::new(&args);
    assert_eq!(config.query, "a");
    assert_eq!(config.file_path, "b");
}

#[test]
fn build_this() {
    let args: Vec<String> = vec![String::from("0"), String::from("a"), String::from("b")];

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    assert_eq!(config.query, "a");
    assert_eq!(config.file_path, "b");
}
