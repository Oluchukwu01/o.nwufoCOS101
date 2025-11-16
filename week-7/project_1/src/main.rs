use std::io;
fn area_of_trapezium(h: f64, b1: f64, b2: f64) -> f64 {
    h / 2.0 * (b1 + b2)
}

fn area_of_rhombus(d1: f64, d2: f64) -> f64 {
    0.5 * d1 * d2
}

fn area_of_parallelogram(b: f64, a: f64) -> f64 {
    b * a
}

fn area_of_cube(l: f64) -> f64 {
    6.0 * l * l
}

fn volume_of_cylinder(r:f64, h:f64) -> f64 {
    std::f64::consts::PI * r * r * h
}

fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}

fn main() {
    println!("Choose a shape to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let answer: i32 = read_input("Enter your choice (1-5):") as i32;

    let answer = match answer {
        1 => {
            let h = read_input("Enter the height:");
            let b1 = read_input("Enter base1:");
            let b2 = read_input("Enter base2:");
            area_of_trapezium(h, b1, b2)
        }
        2 => {
            let d1 = read_input("Enter diagonal1:");
            let d2 = read_input("Enter diagonal2:");
            area_of_rhombus(d1, d2)
        }
        3 => {
            let b = read_input("Enter base:");
            let a = read_input("Enter altitude:");
            area_of_parallelogram(b, a)
        }
        4 => {
            let l = read_input("Enter the length:");
            area_of_cube(l)
        }
        5 => {
            let r = read_input("Enter the radius:");
            let h = read_input("Enter the height:");
            volume_of_cylinder(r, h)
        }
        _ => {
            println!("Invalid choice!");
            return;
        }
    };

    println!("The answer is: {}", answer);
}
