fn main(){
	let principal:f64 = 520000000.00;
	let time:f64 = 5.0;
	let rate:f64 = 10.0;

	let mut amount = principal*(1.0+(rate/100))*time;

	let ci = amount - principal;

	println!("This is the total amount after 5 years {}",amount);
	println!("This is the Compund interest after 5 years: {}",ci);
}