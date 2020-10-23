use crate::helper;
use std::collections::HashMap;

pub fn run() {
    let mut engineering: Vec<String> = Vec::new();
    let mut sales: Vec<String> = Vec::new();
    let mut employees: HashMap<String, String> = HashMap::new();

    loop {
        println!("Welcome, what would you like to do?:");
        println!("1) Add user to department");
        println!("2) List Engineering");
        println!("3) List Sales");
        println!("4) List All");
        println!("5) Exit");

        let option = helper::get_user_input();

        if option == "1" {
            println!("Who would you like to add?:");
            println!("<Add> <User> to <Department>");

            let input = helper::get_user_input();
            let words: Vec<&str> = input.split_whitespace().collect();


            let action = String::from(words[0]);
            let user = String::from(words[1]);
            let department = String::from(words[3]);

            if action == "add" {
                if department == "engineering" {
                    engineering.push(user.clone());
                } else if department == "sales" {
                    sales.push(user.clone())
                }
                employees.insert(user, department);
            };
        } else if option == "2" {
            println!("Engineering department:");
            &engineering.sort();
            for engineer in &engineering {
                println!("{}", engineer)
            }
        } else if option == "3"  {
            println!("Sales department:");
            &sales.sort();
            for salesperson in &sales {
                println!("{}", salesperson)
            }
        } else if option == "4"  {
            println!("All Employees");
            for (employe, department) in &employees {
                println!("{}: {}", employe, department);
            }
        } else {
            println!("Not a valid option, please try again.");
            continue
        }
    }
}


