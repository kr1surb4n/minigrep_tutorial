use std::env;

fn main() {
    // get all command line args and load them into structure
    let args: Vec<String> = env::args().collect();
    // will panic on non-unicode sign
    // std::env::args_os -> unicode safe


    dbg!(args);
    // args = [
    // "target/debug/minigrep",
    // "searchstring",
    // "example-filename.txt",
    // ]


}
