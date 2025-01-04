
   use std::io::Write;
use std::fs::File;
fn main() {
    println!("Welcome");

    let commisioner = vec!["Aigbogun Alamba Daudu",
    "Murtala Afeez Bendu",
    "Okoracha",
    "Adewale Jimoh Akanbi",
    "Osazuwa Faith Etieye"
    ];

    let ministry = vec!["Internal Affairs",
    "Justice",
    "Defense",
    "Power and steel",
    "Petroleum"

    ];
    let geopolitical_zone = vec![
      "South west",
      "North east",
      "South south",
      "South west",
      "South east"

    ];
    let mut file = std::fs::File::create("mimii.txt").expect("create failed");

    file.write_all("commisioner:\n".as_bytes()).unwrap();
    file.write_all(commisioner.join("\n").as_bytes()).unwrap();
    file.write_all("\n\nministry:\n".as_bytes()).unwrap();
    file.write_all(ministry.join("\n").as_bytes()).unwrap();
    file.write_all("\n\ngeopolitical_zone:\n".as_bytes()).unwrap();
    file.write_all(geopolitical_zone.join("\n").as_bytes()).unwrap();
 
    println!(" categories saved to mimii.txt");
    
}