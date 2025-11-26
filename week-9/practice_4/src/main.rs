use std::fs::OpenOptions;
use std::io::Write;


fn main() {
     let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of Data Science";


    let mut file = std::fs::File::create("data.txt").expect("create failed");
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("Write failed");
    file.write_all("\nThis is the appendage to the document.txt."
        .as_bytes()).expect("Write failed");
    println!("file append success");
}