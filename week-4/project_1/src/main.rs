use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not avalid number");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut input3).expect("Not avalid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d:f32 = b * b - 4.0 * a * c;
    println!("Discriminant is {}", d);

    if d > 0.0 {
        let root1:f32 = (-b + d.sqrt()) / (2.0 * a);
        let root2:f32 = (-b - d.sqrt()) / (2.0 * a);
        println!("x1 = {}", root1);
        println!("x2 = {}", root2);
    } 
    else if d == 0.0 {
        let root:f32 = -b / (2.0 * a);
        println!(" x = {}", root);
    }
    else {
        let real_root:f32 = -b / (2.0 * a);
        let imag_root:f32 = -d.sqrt() / (2.0 * a);
        println!("complex roots:");
        println!("x1 = {} + {}i", real_root, imag_root);
        println!("x2 = {} - {}i", real_root, imag_root );
    }

}