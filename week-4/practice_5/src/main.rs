// Rust program to read the height of a person
//and then print if the person is tall, dwarf,
//average in height.

use std::io;
fn main()
{
    let mut input = String::new();

    println!("\nEnter you height (in centimetres):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid string");

    if height >= 150.0 && height <= 170.0
    {
        println!("You're an average height person");
    }
    else if height > 170.0 && height <= 195.0
    {
        println!("You're tall");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You're a dwarf");
    }
    else {
        println!("Abnormal height");
    }
}