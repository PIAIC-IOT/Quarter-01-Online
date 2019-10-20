use std::collections::HashMap;


fn main() {

let field_key = 50;

let field_value = 1000;

let mut map = HashMap::new();

map.insert(field_key, field_value);

println!("{:?}",map);


println!("{}",field_key);

println!("{}",field_value);

}
