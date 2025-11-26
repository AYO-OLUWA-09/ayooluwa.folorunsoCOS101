use::std::io::Write;


fn main() {
    // Names of commisioners
    let names = vec![
    "Aigbogun Alamba Daudu",
    "Murtala Afeez Bendu",
    "Okorocha Calistus Ogbona",
    "Adewale Jimoh Akanbi",
    "Osazuwa Faith Etieye",
    ];

    // Ministry

    let ministry = vec![
    "Internal Affairs",
    "Justice",
    "Defence",
    "Power & Steel",
    "Petroleum",
    ];

    // geopolitical Zones


    let g_zones = vec![
    "South West",
    "North East",
    "South South",
    "South West",
    "South East",
    ];

    // creating files

     let mut file = std::fs::File::create("Ministers Data.txt").expect("create failed");

     println!("MINISTERS DATA");

     // merging datas using index in arrays

     for i in 0..names.len() {

        let datas = format!(
            "{}.\nNames: {}\nMinistry: {}\nGeopolitical Zones: {}\n\n", i+1, names[i], ministry[i], g_zones[i]);

     
     // printing to console
     print!("{}", datas);


     // writing into file 
      file.write_all(datas.as_bytes()).expect("write failed");
    }

    println!("Ministers Data successfully updated");


}