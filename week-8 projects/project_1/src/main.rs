use std::io;


fn main() {
    

    let staff_levels = vec![

    // office administrator
    ("Office Administrator", 0, 2, "APS 1-2", vec!["Intern"]),
    ("Office Administrator", 3, 5, "APS 3-5", vec!["Administrator"]),
    ("Office Administrator", 5, 8, "APS 5-8", vec!["Senior Administrator"]),
    ("Office Administrator", 8, 10, "EL1", vec!["Office Manager"]),
    ("Office Administrator", 10, 13, "EL2", vec!["Director"]),
    ("Office Administrator", 14, 40, "SES", vec!["CEO"]),

        // Academics

    ("Academic", 0, 3, "APS 1-3", vec!["-"]),
    ("Academic", 4, 7, "APS 4-7", vec!["Research Assistant"]),
    ("Academic", 8, 12, "APS 8-12", vec!["PhD candidates"]),
    ("Academic", 13, 15, "EL1", vec!["Post-Doc Researcher"]),
    ("Academic", 16, 20, "EL2", vec!["Senior Lecturer"]),
    ("Academic", 21, 40, "SES", vec!["Dean"]),


    // Lawyers
    ("Lawyer", 0, 2, "APS 1-2", vec!["Paralegal"]),
    ("Lawyer", 3, 5, "APS 3-5", vec!["Junior Associate"]),
    ("Lawyer", 5, 8, "APS 5-8", vec!["Associate"]),
    ("Lawyer", 8, 10, "EL1", vec!["Senior Associate 1-2"]),
    ("Lawyer", 10, 13, "EL2", vec!["Senior Associate 3-4"]),
    ("Lawyer", 14, 40, "SES", vec!["Partner"]),

    // Teacher
    ("Teacher", 0, 2, "APS 1-2", vec!["Placement"]),
    ("Teacher", 3, 5, "APS 3-5", vec!["Classroom Teacher"]),
    ("Teacher", 5, 8, "APS 5-8", vec!["Snr Teacher"]),
    ("Teacher", 8, 10, "EL1", vec!["Leading Teacher"]),
    ("Teacher", 10, 13, "EL2", vec!["Deputy Principal"]),
    ("Teacher", 14, 40, "SES", vec!["Principal"]),

 ];

 // getting input

 println!("Enter occupation (Office Administrator, Academic, Lawyer, Teacher");
 let mut input1:Str = String::new();
 io::stdin().read_line(&mut input1).expect("Failed to read input");
 let occupation = input1.trim().parse().expect("Invalid input");


 // years of experience
 println!("Enter your years of experience");
 let mut input2 = String::new();
 io::stdin().read_line(&mut input2).expect("Failed to read input");
 let years = input2.trim().parse().expect("Invalid input");


 // matching levels

 let mut found = false;

  for (occupation, min_years, max_years, level, positions) in &staff_levels {
        if *occupation == *occupation && years >= *min_years && years <= *max_years {
            println!("\nStaff Level: {}", level);
            println!("Possible Positions:");
            for pos in positions {
                println!("- {}", pos);
            }
            found = true;
            break;
        }
         if !found {
        println!("\nNo matching level found for this occupation and years of experience.");
    }
}



}



    
