fn main() {

let four = Some(4);

let result = plus_one(four);

println!("{:?}",result);

let value = plus_one(None);

println!("{:?}",value);
}

fn plus_one(x: Option<i32>)-> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}


