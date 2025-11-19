use std::io;

fn main() {
    const PI:f32 = 3.143;
    const BIRTH_YEAR:i32 = 1981;
    

    println!("Enter present year:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let c_year:i32 = input1.trim().parse().expect("Invalid Input");


   let c_age:i32 = c_year - BIRTH_YEAR;

    println!("Your present age is {}",c_age);

    println!("The value for PI:{}",PI);
}