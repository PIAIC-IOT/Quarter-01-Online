use std::time::Duration;
use std::thread;

fn main() {

let simulated_user_specified_value = 10;
let simulated_random_number = 7;

generate_workout(simulated_user_specified_value, simulated_random_number);

}

fn simulated_expensive_calculation(intensity: u32) -> u{
    println!("calculating slowly");
    thread::sleep(Duration::from_secs(2));
    intensity
}


fn generate_workout(intensity: u32, random_number: u32){

    let expensive_closure = |num|{
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25{
        println!("today do {} pushups",expensive_closure(intensity));

        println!("next do {} situps",expensive_closure(intensity));
    }
    else{
        if random_number == 3{
            println!("take a break today");
        }
        else{
            println!("today, run for {} minutes ", expensive_closure(intensity));
        } 
    }
}

