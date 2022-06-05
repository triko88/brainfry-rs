use std::{env, process, fs};
use brainfry_rs;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("USAGE: brainfry_rs <filename> [--debug]");
        process::exit(1);
    }
    let filename = &args[1];
    let src = fs::read_to_string(filename).expect("Unable to read the file");
    let debug = args.contains(&"--debug".to_owned());

    println!("{}", brainfry_rs::decode(src, debug));
}
