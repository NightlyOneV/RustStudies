//Libs
use std::io;

//Functions
fn start_countdown_reverse(start: i32){
    println!("Starting countdown from {} to 0", start);
    for x in (0..=start).rev() {
        println!("{}", x);
    }
    
    println!("Start!");
}

fn start_countdown(start: i32){
    println!("Starting countdown from 0 to {}", start);
    for x in 0..=start {
        println!("{}",x);
    }
    
    println!("Start!");
}

fn string_to_i32(input: &str) -> i32 {
    if input.trim().is_empty(){
        println!("User input is null, returning 5 as default");
        return 5
    }

    match input.parse::<i32>(){
        Ok(num) => return num,
        Err(_) => {
            println!("Error on parsing input to a valid i32, returning 5 as default");
            return 5
        },
    }

}

fn return_user_input(_msg: Option<&str>) -> String {
    let mut input = String::new();
    
    let msg = match _msg {
        Some(_s) => _s,
        None => "Type something: ",
    };
    
    println!("{}", msg);
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Input: {}", input)
        }
        Err(error) => println!("Error: {}", error)
    }
    
    input.trim().to_string()
    
}

//Main
fn main(){
    let input: i32 = {
        let _i: String = return_user_input(Some("Type a number: "));
        string_to_i32(&_i)
    };
    
    let countdown_choice: String = return_user_input(Some("Select a countdown type:\n1 - Reverse Countdown\n2 - Countdown\nType: "));
    
    match string_to_i32(&countdown_choice) {
        1 => start_countdown_reverse(input),
        2 => start_countdown(input),
        _ => {
            println!("Not a valid countdown type, going with Countdown (2) as default.");
            start_countdown(input);
        }
    }
    
}