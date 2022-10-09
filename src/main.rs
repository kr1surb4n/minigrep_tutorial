use minigrep::Config;
use std::env;
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
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
