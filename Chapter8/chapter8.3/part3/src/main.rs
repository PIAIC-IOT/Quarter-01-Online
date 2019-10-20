use std::collections::HashMap;

fn main() {

    let teams = vec![String::from("Blue"), String::from("Yellow"),String::from("Green")];

    let initial_scores = vec![10,50,150,60];


    let map: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}",map);
}
