// rust program to compute shapes and solve their areas
use std::io;

fn main() {
    println!("Select shape to compute:");
    println!("1. Area of a Trapezium:");
    println!("2. Area of the Rhombus:");
    println!("3. Area of a Parallelogram:");
    println!("4. Area of a Cube:");
    println!("5. Volume of a cylinder:");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let selected:u32 = input1.trim().parse().expect("Invalid input");


    let result = match selected {
        1 => a_trapezium(),
        2 => a_rhombus(),
        3 => a_parallelogram(),
        4 => a_cube(),
        5 => v_cylinder(),
        _ => {
            println!("Invalid choice!");
            return;
        }
    };
    println!("Result is: {}",result);
}


fn a_trapezium(area_t:f64) -> f64 {
    println!("Enter height: ");
    let height = String::new();
    io::stdin().read_line(&mut height).expect("Not a valid String");
    let height:f64 = height.trim().parse().expect("Invalid String");

    println!("Enter base1:");
     let base1 = String::new();
    io::stdin().read_line(&mut base1).expect("Not a valid String");
    let base1:f64 = base1.trim().parse().expect("Invalid String");

     println!("Enter base2:");
     let base2 = String::new();
    io::stdin().read_line(&mut base2).expect("Not a valid String");
    let base2:f64 = base2.trim().parse().expect("Invalid String");

  let  area_t = height / 2.0 * (base1 + base2);
    println!("Area of Trapezium: {}",area_t);
}

fn a_rhombus(area_r: f64) ->f64 {
    println!("Enter diagonal1:");
    let mut d1 = String::new();
    io::stdin().read_line(&mut d1).expect("Not a valid String");
    let d1:f64 = d1.trim().parse().expect("Invalid String");

     println!("Enter diagonal2:");
    let mut d2 = String::new();
    io::stdin().read_line(&mut d2).expect("Not a valid String");
    let d2:f64 = d2.trim().parse().expect("Invalid String");

    let area_r = 0.5 * d1 *d2;
    println!("Area of Rhombus: {}",area_r);
}

fn a_parallelogram(area_p: f64) ->f64 {
     println!("Enter base:");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Not a valid String");
    let base:f64 = base.trim().parse().expect("Invalid String");

     println!("Enter altitude:");
    let mut altitude = String::new();
    io::stdin().read_line(&mut altitude).expect("Not a valid String");
    let altitude:f64 = altitude.trim().parse().expect("Invalid String");

    let area_p = base * altitude;
    println!("Area of a Parallelogram: {}",area_p);
}

fn a_cube(area_c: f64) ->f64 {
     println!("Enter sidee:");
    let mut side = String::new();
    io::stdin().read_line(&mut side).expect("Not a valid String");
    let side:f64 = side.trim().parse().expect("Invalid String");

    let area_c = 6.0 * side * side;
    println!("Area of Cube: {}",area_c);
return side
}

fn v_cylinder(v_c:f64) ->f64 {
     println!("Enter radius:");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("Not a valid String");
    let radius:f64 = radius.trim().parse().expect("Invalid String");

     println!("Enter height:");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Not a valid String");
    let height:f64 = height.trim().parse().expect("Invalid String");

    const PI:f64 = 3.142;

    let v_c:F64 = PI * radius * radius * height;
    println!("Volume of a cylider: {}",v_c);






}