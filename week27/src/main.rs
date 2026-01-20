fn main() {
    let ans = find_first_a(String::from("Malik Amaaan"));
    match ans {
        None => println!("a is not found"),
        Some(val) => println!("a fount at index {}", val),
    };
}

fn find_first_a(str: String) -> Option<i32> {
    let mut index = 0;
    for c in str.chars() {
        if c == 'a' {
            return Some(index);
        };
        index = index + 1;
    }
    None
}