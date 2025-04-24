use std::{io::stdin, process};
use count::count_lines;

fn main() {
    let lines = count_lines(stdin().lock());
    match lines {
        Ok(lines) => println!("{lines} lines"),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}
