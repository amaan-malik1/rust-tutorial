use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy)]

struct User {
    username: String,
    password: u32,
}

fn main() {
    let u = User {
        username: String::from("Amaan"),
        password: 389482,
    };

    let mut v: Vec<u8> = Vec::new();

    let ans = u.serialize(&mut v);
    match ans {
        Ok(_) => println!("{:?}", v),
        Err(_) => println!("Error while serializing"),
    }
}
