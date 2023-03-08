use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // dbg!(&args);
    // args = [
    // "target/debug/minigrep",
    // "searchstring",
    // "example-filename.txt",
    // ]

    // get all command line args (env::args())
    // will panic on non-unicode sign
    // std::env::args_os -> unicode safe
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
