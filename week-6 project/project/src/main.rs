use std::io;
fn main() {
println!(" Menu               Price
    P = Yam/Edinkaiko Soup  - N3_200");
println!(" Menu               Price
    F = Fried Rice & Chicken - N3_000");
println!(" Menu               Price
    A = Amala & Ewedu Soup  - N2_500");
println!(" Menu               Price
    E = Eba & Egusi Soup    - N2_000");
println!(" Menu               Price
    W = White Rice & Stew   - N2_500");

//taking the orders

let mut input1 = String::new();
let mut input2 = String::new();
//let input3 = String::new();

println!("Enter your order code,please: ");
io::stdin().read_line(&mut input1).expect("Not a valid string");


println!("Input Quantity: ");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let qantity:i32 = input2.trim().parse().expect("Not a valid input");


let price = match input1.as_str() {
    "P" => 3_200,
    "F" => 3_000,
    "A" => 2_500,
    "E" => 2_000,
    "W" => 2_500,
    &_ => todo!(),
};
let mut total:i32 = qantity * price;

// discount

if total > 10_000 
{

println!("Hurray, you got a 5% discount");

total  = total * (5 / 100);
}


println!("Your bill is {}",total);

}


