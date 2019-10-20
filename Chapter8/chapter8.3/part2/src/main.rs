use std::collections::HashMap;

fn main() {

    let mut map = HashMap::new();

    map.insert(String::from("Blue"), 10);

    map.insert(String::from("Yellow"), 50);

    println!("{:?}",map);


}
