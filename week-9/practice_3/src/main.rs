use std::fs;
use std::io::Write;

fn main() {
     let name = "Folorunso Ayooluwa";
    let dept = "Department of Data Science";


    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n"
        .as_bytes()).expect("write failed");
    file.write_all(name.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\nData written to file.");


    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}