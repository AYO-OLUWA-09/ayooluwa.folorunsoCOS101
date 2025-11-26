use std::io::Write;

fn main() {
    // inputing datas
    let company_name = "Nigeria Brewries Plc\n";
    let est = "Incoporated in 1946";
    let record = "The pioneer and largest brewry company in Nigeria";

    let lager = "Lager:\n33 Export\nDesperados\nGoldberg\nGulder\nHeineken\nStar\n\n";
    let stout = "Stout:\nLegend\nTubor King\nWilliams\n\n";
    let non_alcoholic = "Non-Alcholic:\nMaltina\nAmstel Malta\nMalta Gold\nFayrouz\n\n";


// creating the file
    let mut file = std::fs::File::create("drinks.txt").expect("create failed");

    // writing file

     file.write_all(company_name.as_bytes()).expect("write failed");
     file.write_all(est.as_bytes()).expect("write failed");
     file.write_all(record.as_bytes()).expect("write failed");
     file.write_all(lager.as_bytes()).expect("write failed");
     file.write_all(stout.as_bytes()).expect("write failed");
     file.write_all(non_alcoholic.as_bytes()).expect("write failed");

     println!("\nData created for Nigeria Brewries");
}
