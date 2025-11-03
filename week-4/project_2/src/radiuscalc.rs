use std::io;

fn main(){

	let mut input1 = String::new();

	println!("Enter the radius: ");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let r:f32 = input1.trim().parse().expect("Not a valid number");

	let pi:f32 = 22.0 / 7.0;
	let area:f32 = pi * r * r;
	println!("Area = {}", area);
}