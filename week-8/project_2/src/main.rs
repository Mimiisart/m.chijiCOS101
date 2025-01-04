use std::fs::File;
use std::io::Write;

fn main() {
    let student_name = vec![
      "Oluchi Mordi",
      "Adams Aliyu",
      "Shania Bolade",
      "Adekunle Gold",
      "Blanca Edemoh",
    ];

    let matric_number = vec![
        "ACC10211111",
        "ECO10110101",
        "CSC10228828",
        "EEE11020202",
        "MEE10202001",
    ];

    let department = vec![
        "Accounting",
        "Economics",
        "Computer",
        "Electrical",
        "Mechanical",
    ];
    let level = vec![
        "300",
        "100",
        "200",
        "200",
        "100",
    ];

    

    let mut file = std::fs::File::create("mimii.txt").expect("create failed");
    println!("Student name,\tMatric number,\tDepartment,\tLevel\t");
    file.write_all("Student name,\tMatric number,\tDepartment,\tLevel\t\n".as_bytes()).unwrap();

    for x in 0..5{
        println!("{},\t{},\t{},\t{}\t",student_name[x],matric_number[x],department[x],level[x]);
        file.write_all(format!("{},\t{},\t{},\t{}\t\n",student_name[x],matric_number[x],department[x],level[x]).as_bytes()).unwrap();
    }
    
}
