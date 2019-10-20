fn main() {
    

    let v = vec![2,4,6,8,10];

    let element1 = v[2];

    println!("{}",element1);


    let element2 = &v[2];

    println!("{}",element2);


    let element3 = v.get(2);

    println!("{:?}",element3);


    match element3{
        Some(value) => println!("{}",value),
        None => println!("nothing")
    }
}
