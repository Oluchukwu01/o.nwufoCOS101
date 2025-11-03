use std::io;

fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

//prompt to enter values
    println!("Enter the value of P: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of R: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of T: ");
    io::stdin().read_line(&mut input3).expect("Not a valid number");
    let c:f32 = input3.trim().parse().expect("Not a valid number");
    
    //calculating amount

    let mut amt = a * (1.0 + (b / 100.0)) *  c;
    let mut cl = amt - a;

    while amt < 10.0 {
        println!("inside loop amount value is {}", amt);
        amt+=1.0;
    }
    println!("outside loop amount value is {}", amt);

    while cl < 10.0{
        println!("inside loop compound interest value is {}", cl);
        cl+=1.0;
    }
    println!("outside loop compound interest value is {}", cl);
}