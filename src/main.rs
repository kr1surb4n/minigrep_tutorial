use std::env;
use std::fs;

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

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}