use std::collections::HashMap;

fn main() {

    let mut map = HashMap::new();

    map.insert(String::from("Blue"), 10);

    map.insert(String::from("Yellow"), 50);

    let access_key = String::from("PIAIC");

    let result = map.get(&access_key);

    println!("{:?}",result);
}