extern crate rand;

use std::cmp::Ordering;
use rand::Rng;

mod helper;
mod temp_converter;

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
            temp_converter::convert();
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