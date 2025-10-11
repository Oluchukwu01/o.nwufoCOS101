fn main () {
	let t:f64 = 450000.0 * 2.0; 
	let m:f64 = 1500000.0;
	let h:f64 = 750000.0 * 3.0;
	let d:f64 = 2850000.0 * 3.0;
	let a:f64 = 250000.0;
	// total amount of sales made
	let s = t + m + h + d + a;
	println!("Total amount of sales made is {}", s);
	let to:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;
	let av = s / to;
	println!("Average of sales made is {}", av);
	


}