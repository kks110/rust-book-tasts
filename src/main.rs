extern crate rand;

use std::cmp::Ordering;
use rand::Rng;


mod helper;

fn main() {
    loop {
        println!("Welcome, what would you like to do?:");
        println!("1) Guessing Game");
        println!("2) Convert temperatures");
        println!("3) Fibonacci");
        println!("4) Twelve Days of Christmas");

        let option = helper::get_user_input();

        if option == "1" {
            guessing_game();
            break
        } else if option == "2" {
            temp_converter();
            break
        } else if option == "3" {
            fibonacci();
            break
        } else if option == "4"  {
            twelve_days_of_christmas();
            break
        } else {
            println!("Not a valid option, please try again.");
            continue
        }
    }
}

fn temp_converter() {
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


fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let guess: u32 = helper::get_user_input_unumber();

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win");
                break;
            },
        }
    }
}

fn fibonacci() {
    println!("How many Fibonacci numbers would you like to see?");
    let nth = helper::get_user_input_unumber();
    let mut previous: u128 = 0;
    let mut current: u128 = 1;
    let mut next: u128;
    for i in 0..nth {
        println!("{}: {}", i+1, current);
        next = previous + current;
        previous = current.clone();
        current = next;
    }
}

fn twelve_days_of_christmas() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let items = [
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings, badam-pam-pam",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for i in 0..days.len() {
        println!();
        println!("On the {} day of christmas", days[i]);
        println!("My true love gave to me:");
        if i == 0 {
            println!("A partridge in a pear tree")
        } else {
            for j in 0..i {
                println!("{}", items[i-j]);
            }
            println!("{}", items[0]);
        }
    }

}