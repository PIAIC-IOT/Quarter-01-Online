fn main() {

    // let my_coin = Coin::Penny;

    // let value = value_in_cents(my_coin);

    // println!("{}",value);



    let my_coin = Coin::Dime;

    let value = value_in_cents(my_coin);

    println!("{}", value);





//     let my_coin = Coin::Quarter(UsState::Alabama);

//     let value = value_in_cents(my_coin);

//     println!("{}",value );
}


#[derive(Debug)]
enum Coin{

    Penny,
    Nickel,
    Dime,
    Quarter(UsState),

}

#[derive(Debug)]
enum UsState{

    Alabama,
    Alaska,

}

fn value_in_cents(coin: Coin)->u8{

    match coin {

        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25
    }

}

// fn value_in_cents(coin: Coin)->u8{

//     match coin {

//         Coin::Penny => {
//                 println!("Lucky penny");
//                 1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("{:?}", state);
//             25
//         }

//     }

// }
