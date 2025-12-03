// declare a stucture
#[derive(Debug)]
struct Employee {
    ceo:String,
    company:String,
    age:u32
}
fn main() {
    // inisilize a structure

    let emp1 = Employee {
        company:String::from("Microsoft Coporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };
    let emp2 = Employee {
        company:String::from("Google Inc."),
        ceo:String::from("Sundia Pichai"),
        age:51
    };
    // pass emp1 and emp2
    display(emp1);
    display(emp2);

}

fn display(emp:Employee){
    println!("Name is: {} company is {}, age is {}", emp.ceo, emp.company, emp.age);
}