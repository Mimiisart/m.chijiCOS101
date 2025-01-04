use std::io::Write;
use std::fs::File;
fn main() {
    println!("Welcome to the Nigeria brewery limited,here are the list of high quality drinks\n");

    let lager = vec![
        "33 export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star"

    ];
     let stout = vec!["legend","Turbo king","Williams"];

     let non_alcoholic = vec!["Maltina","Amstel malta","Malta gold","Fayrouz"];
    
    let mut file = std::fs::File::create("mimii.txt").expect("create failed");
    

    file.write_all("Lager:\n".as_bytes()).unwrap();
    file.write_all(lager.join("\n").as_bytes()).unwrap();
    file.write_all("\n\nstout:\n".as_bytes()).unwrap();
    file.write_all(stout.join("\n").as_bytes()).unwrap();
    file.write_all("\n\nnon_alcoholic:\n".as_bytes()).unwrap();
    file.write_all(non_alcoholic.join("\n").as_bytes()).unwrap();
 
    println!("Drink categories saved to drink.txt");
 
    
}
