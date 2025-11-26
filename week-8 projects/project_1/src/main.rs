use std::io;


fn main() {
    // FOR TE OCCUPATIONS

    let office_administrator = vec![("APS 1-2", "Intern"), 
                               ("APS 3-5", "Administrator"),
                               ("APS 5-8", "Senior Administrator"),
                               ("EL1 8-10", "Office Manager"),
                               ("EL2 10-13", "Director"),
                               ("SES", "CEO"),
                                ];

    let 

 // getting input

 println!("Enter occupation (Office Administrator, Academic, Lawyer, Teacher");
 let mut occupation_input = String::new();
 io::stdin().read_line(&mut occupation_input).unwrap();
 let occupation = occupation_input.trim();


 // years of experience
 println!("Enter your years of experience");
 let mut years_input = String::new();
 io::stdin().read_line(&mut years_input).expect("Failed to read input");
 let years:i32 = years_input.trim().parse().expect("Invalid input");


 // matching levels

 let mut found = false;

  for (occupation, min_years, max_years, level, positions) in &staff_levels {
        if occupation.to_lowercase() == occupation_input.to_lowercase()
         && years >= *min_years
         && years <= *max_years 
    {
            println!("\nStaff Level: {}", level);
            println!("Possible Positions:");
            for pos in positions {
                println!("- {}", pos);
            }
            found = true;
            break;
        }
        }
        if !found
}







    
