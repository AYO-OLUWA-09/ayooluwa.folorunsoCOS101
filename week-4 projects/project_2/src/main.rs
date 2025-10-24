// Rust program to determine the incentives
// of employees based on status and age


use std::io;
fn main() {
let mut experience = String::new();
let mut age = String::new();


// Experience 

println!("Are you experienced ? (yes / no): ");
io::stdin().read_line(&mut experience).expect("Unreadable input");


// Age

println!("Enter your age: ");
io::stdin().read_line(&mut age).expect("Unreadable input");
let age:i32 = age.trim().parse().expect("Enter a valid number");

let a:i64 = 1_560_000;
let b:i64 = 1_480_000;
let c:i64 = 1_300_000;
let d:i64 = 100_000;

// Incentives
if  age >= 40 
{
    println!("Incentive is: {}", a);
}
else if age < 40 && age >= 30
{
    println!("Incentive is: {}", b);
}
else if  age < 28 
{
    println!("Incentive is: {}", c);
}
else 
{
    println!("Incentive is: {}", d);
}

}

