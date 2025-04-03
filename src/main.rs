use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
      panic!("Incorrect usage. Usage: cargo run -- file.txt");
    }
    let file = get_file(&args[1]);
    println!("{:?}", file);

    let reader = BufReader::new(file);
    for line in reader.lines() {
      match line {
        Ok(line) => println!("{}", line),
        Err(e) => println!("Error reading line: {}", e)
      }
    }
}

fn get_file(path: &String) -> File {
  let file = File::open(path);
  match file {
    Ok(file) => file,
    Err(e) => panic!("{:?}", e)  
  }
}
