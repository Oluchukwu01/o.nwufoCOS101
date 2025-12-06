#[derive(Debug)]
struct Laptop {
    brand:String,
    price:u64,
    qty:u16
}
fn main() {
    let lap1 = Laptop {
        brand:String::from("HP"),
        price:650_000,
        qty:10
    };
    let lap2 = Laptop {
        brand:String::from("IBM"),
        price:755_000,
        qty:6
    };
    let lap3 = Laptop {
        brand:String::from("Toshiba"),
        price:550_000,
        qty:10
    };
    let lap4 = Laptop {
        brand:String::from("Dell"),
        price:850_000,
        qty:4
    };
    let total = 3 * lap1.price + 3 * lap2.price + 3 * lap3.price + 3 * lap4.price;
    println!("The laptops purchased are {}, {}, {}, {}",
        lap1.brand,lap2.brand,lap3.brand,lap4.brand);
    println!("The quantity available for each laptop is:\n HP: {}\n IBM: {}\n Toshiba {}\n Dell:{}",
        lap1.qty,lap2.qty,lap3.qty,lap4.qty);
    println!("The price of each laptop is:\n HP: {}\n IBM: {}\n Toshiba {}\n Dell:{}",
        lap1.price,lap2.price,lap3.price,lap4.price);
    println!("The total price of the laptops purchased is {}",total);
}