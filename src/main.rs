
use std::io::{self, Read};
use std::fs::File;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось считать строку");
    match File::open(input.trim()) {
        Ok(mut f) => {
            let mut buf = Vec::new();
            if f.read_to_end(&mut buf).is_ok() {
                println!("success");
            } else {
                println!("failure");
            }
        }
        Err(_) => {
            println!("failure");
        }
    } 
}
