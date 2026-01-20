// fn main() {
//     let s = print_var(String::from("Amaan"), String::from("Malik"));
// }

// fn print_var<T: std::fmt::Display>(a: T, b: T) {
//     println!("{}", a);
//     println!("{}", b);
// }

// fn main() {
//     let res = biggest_num(102, 23);
//     println!("{}", res);
// }

// fn biggest_num<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
//     if a > b {
//         return a;
//     }
//     return b;
// }

// use std::process::Output;

// struct Rect<T> {
//     width: T,
//     height: T,
// }

// impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
//     fn area(&self) -> T {
//         return self.width * self.height;
//     }
// }

// fn main() {
//     let r = Rect {
//         width: 20,
//         height: 20,
//     };

//     println!("{}", r.area());
// }

trait Shape {
    fn area(&self) -> f32;
}

struct Rect {
    width: f32,
    height: f32,
}

impl Rect {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }
}

fn main() {
    let r = Rect{
        width:10.0,
        height:20.3,
    };

    
}
