use std::io;

fn main(){
    println!("Code           Item           Price
        L          Laptop          555_000
        M          Monitor         120_000
        K          Keyboard        15_000
        H          Headset         25_000");
}
//prompting user to enter  item code and quantity

    let mut input1 = String::new();
    let mut 1nput2 = String::new();

    println("Enter the item code: ");
    io::stdin().read_line(&mut input1).expect("Cannot read line");
    let a:f32 = input1.trim().parse().expect("Cannot read line");

    println("Enter the quantity you want: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    //calculating the total cost

    let l:f32 = 555000.0;
    let m:f32 = 120000.0;
    let k:f32 = 15000.0;
    let h:f32 = 25000.0;

    let total = l * a;

    if total > 500_000.0;
    let total = a * b - (a * b * 0.07);

    println!("Total amount = {}", total);
