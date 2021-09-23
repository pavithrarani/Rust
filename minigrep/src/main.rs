use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let query = &args[1];
    let filename = &args[2];

    println!("query {} filename {}!",query, filename);

    let contents = fs::read_to_string(filename).expect("something wrong reading the file");

    println!("text content\n{}", contents);
}
