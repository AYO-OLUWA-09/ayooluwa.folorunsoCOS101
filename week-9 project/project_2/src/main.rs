use std::io::Write;

fn main() {
    
    // creating the data of the studens making use vector

     println!("PAU-SIMS");

    let stu_data = vec![
    ("1.", "Oluchi Mordi", "ACC10211111", "Accounting", "300 Level"),
    ("2.", "Adams Aliyu", "ECO10110101", "Economics", "100 Level"),
    ("3.", "Shania Bolade", "CSC10328828", "Computer", "200 Level"),
    ("4.", "Adekunle Gold", "EEE11020202", "Electrical", "200 Level"),
    ("5.", "Blanca Edemoh", "MEE10202001", "Mechanical", "100 Level"),
    ];

    // crearing the students file
     let mut file = std::fs::File::create("Students Data.txt").expect("create failed");


     println!("Students Data");

     // looping through

     for stu_data in stu_data.iter() {
     let(no, name, matric_no, dept, level) = stu_data;

        // building the records

        let record = format!(
        "{}\nName: {}\nMatric No: {}\nDepartment: {}\nLevel: {}\n\n",no, name, matric_no, dept, level
            );

        // display 

        print!("{}", record);

        // writing as a file
        file.write_all(record.as_bytes()).expect("write failed");
     }

     println!("Students records successfully updated");
}