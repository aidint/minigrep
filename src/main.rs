use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = args.get(1).expect("Please provide a query");
    let file_path = args.get(2).expect("Please provide a file path");
    
    println!("Searching for {query} in file {file_path}");

    let contents = fs::read_to_string(file_path).expect("File should be readable.");
    println!("With text:\n{contents}");
}
