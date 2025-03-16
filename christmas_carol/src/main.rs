fn main() {
    let christmas_gifts = [
        "A partridge in a pear tree!",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for day in 0..12 {
        println!("\r\nOn the {} day of Christmas my true love sent to me ... ", days[day]);
        for gift in (0..day + 1).rev() {
            let mut gift_string = String::new();
            if day != 0 && gift == 0 {
                gift_string.push_str("...And ");
            }
            println!("{}{}", gift_string, christmas_gifts[gift]);
        }
        println!();
    }

}
