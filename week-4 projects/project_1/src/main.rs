// Rust program to find quadratic roots
// using a=1, b=4, c=4
use std::io;

fn main() {



let mut input1 = String::new();
let mut input2 = String::new();
let mut input3 = String::new();

println!("Enter the value of a: ");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let a:f32 = input1.trim().parse().expect("Enter a valid number");


println!("Enter the value of b: ");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let b:f32 =input2.trim().parse().expect("Enter a valid number");


println!("Enter the value of c: ");
io::stdin().read_line(&mut input3).expect("Not a valid string");
let c:f32 = input3.trim().parse().expect("Enter a valid number");

//find determinant

let mut determinant = (b * b) - (4.0 * a * c);
determinant = determinant.sqrt();

println!("\nThe determinant is {}",determinant);


if determinant > 0.0 {
    let froot = (-b - determinant.sqrt()) / (2.0 * a);
    let sroot = (-b + determinant.sqrt()) / (2.0 * a);
    println!("R1 = {}",froot);
    println!("R2 = {}",sroot);
}
else if determinant == 0.0 {
    let root = -b / (2.0 * a);
println!("Real root = {}",root);
}
else {
    println!("Negative determinant, no root");
}

}

