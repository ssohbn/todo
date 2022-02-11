use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
  for arg in env::args() {
  if let Ok(lines) = read_lines(arg) {
    for line in lines {
      if let Ok(text) = line {
        let text = text.to_lowercase();
        if text.contains("todo") {
          println!("{}", text);
        }
      }
    }
  }  
  }
  
}

// this function is pasted from rust docs
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
  // panics if it cant open the thing
  let file = File::open(filename)?; 
  Ok(io::BufReader::new(file).lines())
}