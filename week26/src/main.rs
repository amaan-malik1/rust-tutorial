fn main() {
    let name = String::from("Amaan");
    let len = get_name(name);
    println!("length of Amaan is {}", len)
}

fn get_name(s: String) -> usize {
    return s.len();
}
