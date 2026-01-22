// macro_rules! Hello {
//     () => {
//         println!("Hello mittar");
//     };
// }

// fn main() {
//     // println!("Hello, mittar");
//     Hello!();
// }

// #[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     age: u32,
// }

// fn main() {
//     let u = User {
//         username: String::from("Amaan"),
//         email: String::from("amikm@gmial.com"),
//         age: 22,
//     };

//     println!("{:?}", u);
// }

// #[derive(Debug, Copy, Clone)]
// struct User {
//     username: String,
//     age: u32,
// }

// fn main() {
//     let u = User {
//         username: String::from("Aman malik"),
//         age: 22,
//     };

//     let u2 = u;

//     println!("{:?}", u);
//     println!("{:?}", u2);
// }

macro_rules! generate_mul_functions{
   ($($func_name:ident),*)=>{
    $(
        fn $func_name(){
            println!("heelo from {} fucntion..", stringify!($func_name));
        }
    )*
   };
}

generate_mul_functions!(bro, saas, nand);

fn main() {
    bro();
    saas();
    nand();
}
