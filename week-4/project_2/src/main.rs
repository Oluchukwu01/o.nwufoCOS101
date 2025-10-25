use std::io;

fn main () {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Input age of employee: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Input the years of experience of employee: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let experience:f32 = input2.trim().parse().expect("Not a valid number");

    let a:f32 = 100_000.0;
    let b:f32 = 1_300_000.0;
    let c:f32 = 1_480_000.0;
    let d:f32 = 1_560_000.0;


    if experience < 7.0 {
        println!("Inexperienced.");
        println!("Incentive is {}", a);
    }
    else {
        println!("Experienced.");
        if age >= 40.0 {
            println!("Incentive is {}", d);
        }
        else if age < 40.0 && age >= 30.0 {
            println!("Incentive is {}", c);
        }
        else{
            println!("Incentive is {}", b);
        }
    }
}