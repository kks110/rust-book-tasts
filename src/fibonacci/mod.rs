use crate::helper;

pub fn run() {
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