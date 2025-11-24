use std::fs;

fn fn main() {
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}