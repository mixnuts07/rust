use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query: &String = &args[1];
    let filename: &String = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("File Not Found");

    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("something went wrong reading the file.");
}
