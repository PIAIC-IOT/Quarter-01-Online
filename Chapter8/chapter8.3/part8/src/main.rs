use std::collections::HashMap;

fn main() {

let mut map = HashMap::new();

let my_string = "Hello world wonderful world";

for word in my_string.split_whitespace(){

    let count = map.entry(word).or_insert(0);

    *count += 1;

}

println!("{:?}",map);

}
