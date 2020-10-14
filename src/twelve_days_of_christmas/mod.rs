pub fn run() {
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