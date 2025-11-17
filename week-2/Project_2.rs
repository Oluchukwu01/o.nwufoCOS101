fn main(){
	let pricetoshiba:f64 = 450000.0;
	let qtytoshiba:f64 = 2.0;
	let toshiba = pricetoshiba*qtytoshiba;

	let pricemac:f64 = 1500000.0;
	let qtymac:f64 = 1.0;
	let mac = pricemac * qtymac;

	let pricehp:f64 = 750000.0;
	let qtyhp:f64 = 3.0;
	let hp = pricehp * qtyhp;

	let pricedell:f64 = 2850000.0;
	let qtydell:f64 = 3.0;
	let dell = pricedell * qtydell;

	let priceacer:f64 = 250000.0;
	let qtyacer:f64 = 1.0;
	let acer = priceacer * qtyacer;

	let totalqty = qtytoshiba + qtymac + qtyhp + qtyacer + qtydell;

	let sum = toshiba + mac + hp + dell + acer;

	let average = sum / totalqty;

	println!("The sum of the sales record is {}",sum);
	println!("The average of the sales made is {}",average);


}