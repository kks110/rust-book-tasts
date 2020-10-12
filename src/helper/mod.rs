use std::io;

pub fn get_user_input() -> String {
    let mut input = String::new();
    let input: &str = loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                break input.trim();
            }
            Err(_) => {
                println!("Failed to read line");
                continue
            },
        }
    };
    input.to_string()
}

pub fn get_user_input_float() -> f32 {
    let input = loop {
        match get_user_input().trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a number");
                continue
            },
        };
    };
    input
}

pub fn get_user_input_unumber() -> u32 {
    let input = loop {
        match get_user_input().trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a number");
                continue
            },
        };
    };
    input
}
