fn main() {
    
    let some_number = Option::Some(5);

    let some_string = Option::Some(String::from("hi!"));

    println!("{:?}",some_number);

    println!("{:?}",some_string);
}

#[derive(Debug)]
enum Option <T> {
    Some(T),
    None
}