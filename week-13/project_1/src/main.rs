use std::io;
use std::io::Read;

fn main() {
    let mut input1 = String::new();
    println!(
        "What is your role? (1-5)
1) Administrator
2)Project_manager
3)Employee
4)Customer
5)Vendor"
    );
    io::stdin().read_line(&mut input1).expect("failed to input");
    let role: u32 = input1.trim().parse().expect("failed to parse input");

    if role == 1 {
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else if role == 2 {
        let mut file = std::fs::File::open("project_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else if role == 3 {
        let mut file = std::fs::File::open("staff_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else if role == 4 {
        let mut file = std::fs::File::open("customer_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    } else if role == 5 {
        let mut file = std::fs::File::open("dataplan_table.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }
}
