// macro_rules! Hello {
//     () => {
//         println!("Hello mittar");
//     };
// }

// fn main() {
//     // println!("Hello, mittar");
//     Hello!();
// }

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: u32,
}

fn main() {
    let u = User {
        username: String::from("Amaan"),
        email: String::from("amikm@gmial.com"),
        age: 22,
    };

    println!("{:?}", u);
}
