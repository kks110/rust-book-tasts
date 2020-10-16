extern crate rand;

mod helper;
mod temp_converter;
mod guessing_game;
mod fibonacci;
mod twelve_days_of_christmas;
mod mean_mode_median;

fn main() {
    loop {
        println!("Welcome, what would you like to do?:");
        println!("1) Guessing Game");
        println!("2) Convert temperatures");
        println!("3) Fibonacci");
        println!("4) Twelve Days of Christmas");
        println!("5) Enter a list of number and receive the mean, mode and median numbers");

        let option = helper::get_user_input();

        if option == "1" {
            guessing_game::run();
            break
        } else if option == "2" {
            temp_converter::run();
            break
        } else if option == "3" {
            fibonacci::run();
            break
        } else if option == "4"  {
            twelve_days_of_christmas::run();
            break
        } else if option == "5" {
            mean_mode_median::run();
            break
        } else {
            println!("Not a valid option, please try again.");
            continue
        }
    }
}