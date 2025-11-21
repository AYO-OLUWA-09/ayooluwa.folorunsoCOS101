use std::io;

fn main() {
    
    let mut candidates: Vec<(String, u32)> = Vec::new();

    let mut input = String::new();


    println!("How many developers are you accessing?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let count:u32 = input.trim().parse().expect("Enter a number!");


    for _ in 0..count {


        // developers name
         println!("Enter developer's full name");
         let mut input1 = String::new();
         io::stdin().read_line(&mut input1).expect("Failed to read input");
         let names = input1.trim().parse().expect("Invalid String");


         // Years of experience

         println!("Input developer's years of experience");
         let mut input2 = String::new();
         io::stdin().read_line(&mut input2).expect("Failed to read input1");
         let years:i32 = input2.trim().parse().expect("Enter a number");

         // storing a vector as a tuple

         candidates.push((names,years.try_into().unwrap()));

    }

    let mut most_experienced = &candidates[0];

    // looping to find the most experinced

    for candidates in &candidates {

        if candidates.1 > most_experienced.1 {

            most_experienced = candidates;
        }
    }

    println!("\nMost experienced developer is {} with {} years of experience.",most_experienced.0,most_experienced.1);



}
