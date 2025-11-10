fn main() {
    use std::io;
    println!("     FOOD MENU");
    println!("Code\tMenu\t\tPrice (₦)");
    println!("P\tPoundo yam & Edinkaiko Soup\t\t₦3200");
    println!("F\tFried Rice & Chicken\t\t₦3000");
    println!("A\tAmala & Ewedu Soup\t\t₦2500");
    println!("E\tEba & Egusi Soup\t\t₦2000");
    println!("W\tWhite Rice & Stew\t\t₦2500");

    //getting the customer meal input
    println!("Input food code (P,F,A,E,W)");
    let mut code = String::new();
    io::stdin()
    .read_line(&mut code)
    .expect("Failed to read input");
    let code = code.trim().to_uppercase();

    //prompting customer to put quantity
    println!("Input the food quantity");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Not a valid string");
    let quantity:u32 = quantity.trim().parse().expect("Not a valid number");

    //determing the price
    let price = match code.as_str(){
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        &_ => {
            println!("Not a valid food code");
            0
        }
    };
    let total = price * quantity;
    let mut final_amount = total as f32;

    //for the 10% discount
    if total > 10000{
        let discount = final_amount * 0.05;
        final_amount -= discount;
        println!("Your bill is {}",final_amount);
    }
    else {
        println!("Your bill is {}",total);
    }

}