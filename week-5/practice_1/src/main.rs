fn main() {
   let name = "Aisha lawal";
   let uni:&str = "Pan-atlantic University";
   let  addr:&str = "km 52 Lekki-epe Expressway,Ibeju-Lekki,Lagos";
   println!("Name: {}",name);
   println!("University: {}",uni);


   let department:&'static str = "Computer science";
   let school:&'static str = "School of science and technology ";
   println!("Department: {}, \nSchool: {}",department,school);
}
