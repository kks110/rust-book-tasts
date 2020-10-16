use crate::helper;
use std::collections::HashMap;

pub fn run() {
    let mut numbers = get_numbers();

    let mean = mean(&numbers);
    let mode = mode(&numbers);
    let median= median(&mut numbers);

    println!("The Mean is: {}", mean);

    println!("The Median is: {}", median);
    
    if mode.len() == 0 {
        println!("There is no Mode");
    } else if mode.len() == 1 {
        print!("The Mode number is: {}", mode[0]);
    } else if mode.len() > 1 {
        print!("The Mode numbers are: ");
        for number in mode {
            print!("{} ", number)
        }
    }
}

fn get_numbers() -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    println!("Please enter a number, pressing return after each, enter any letter to stop");
    loop {
        match helper::get_user_input_inumber(true) {
            Some(num) => numbers.push(num),
            None => {
                break
            }
        }
    }
    numbers
}

fn mean(numbers: &Vec<i32>) -> i32 {
    let mut accumulator: i32 = 0;
    for number in numbers {
        accumulator += number;
    }
    accumulator / numbers.len() as i32
}

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();
    let mid_point = numbers.len() / 2;
    numbers[mid_point]
}

fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut number_count: HashMap<i32, i32> = HashMap::new();
    for number in numbers {
        let count = number_count.entry(*number).or_insert(0);
        *count += 1;
    }

    let mut mode: Vec<i32> = vec![];
    let mut highest_count: i32 = 0;
    for (number, count) in number_count {
        if count > highest_count && count > 1 {
            highest_count = count;
            mode.clear();
            mode.push(number)
        } else if count == highest_count && count > 1 {
            mode.push(number);
        }
    }
    mode
}