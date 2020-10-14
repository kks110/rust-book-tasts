use crate::helper;

pub fn run() {
    loop {
        println!("Welcome, what would you like to do?:");
        println!("1) Convert to Fahrenheit ");
        println!("2) Convert to Celsius");


        let option = helper::get_user_input();

        if option != "1" && option != "2" {
            println!("Not a valid option, please try again.");
            continue
        };

        println!("What temp would you like to convert?: ");
        let temperature: f32 = helper::get_user_input_float();

        if option == "1" {
            println!("Your temp in fahrenheit is {}", convert_to_fahrenheit(temperature));
            break
        } else if option == "2" {
            println!("Your temp in celsius is {}", convert_to_celsius(temperature));
            break
        }
    }
}

fn convert_to_fahrenheit(temp: f32) -> f32 {
    let multiplicative: f32 = 9.0/5.0;
    (temp * multiplicative) + 32.0
}

fn convert_to_celsius(temp: f32) -> f32 {
    let multiplicative: f32 = 5.0/9.0;
    (temp - 32.0) * (multiplicative)
}