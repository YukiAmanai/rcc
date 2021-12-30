// struct Account { name: String, pass: String }

// fn main() {
//     let a = Account { name: String::from("name"), pass: String::from("pass") };
//     let Account { ref name, ref pass } = a;    // move ownership
//     println!("{} {}", name, pass);     // borrow check!! - OK
//     println!("{} {}", a.name, a.pass); // borrow check!! - Error
// }
