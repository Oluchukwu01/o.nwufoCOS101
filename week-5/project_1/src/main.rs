use std::io;

fn main(){
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();

    //Inputing Student name and grades

    println!("\nEnter Student Name: ");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read input");

    println!("Input grade 1: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let a:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Input grade 2: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let b:f32 = input3.trim().parse().expect("Not a valid number");

    println!("Input grade 3:");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let c:f32 = input4.trim().parse().expect("Not a valid number");

    //calculating the student average

    let avg = (a + b + c) / 3.0;

    if a > 100.0 && a < 0.0 {
        println!("Invalid score");
    }
    if b > 100.0 && b < 0.0 {
        println!("invalid score");
    }
    if c >100.0 &&  c < 0.0 {
        println!("invalid score");
    }
    println!("Hi {}", name);
    println!("Your average score is {}", avg);
}