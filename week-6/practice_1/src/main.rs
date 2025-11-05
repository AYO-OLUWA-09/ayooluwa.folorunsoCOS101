fn main() {
    let name = "AYOOLUWA";
    let uni:&str = "PAU";
    let addr:&str = "km 52 Lekki-epe Expressway,Ibeju-Lekki,Lagos";
    println!("Name: {}",name);
    println!("University; {}, \n Address: {}",uni,addr);

    let department:&'static str = "Data Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {},\nSchool: {}",department,school);

}