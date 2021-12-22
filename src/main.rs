fn main() {
    // exploration of a bunch of conditions and loops
    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is: {}", number);

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);

        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number1 = 3;

    while number1 != 0 {
        println!("{}!", number1);
        number1 -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    println!("{}", celsius_to_fahrenheit(5.));
    println!("{}", fahrenheit_to_celsius(5.));

    println!("{}", generate_nth_fib(0));
    
    christmas_carol();
}

// converts celsius to fahrenheit
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * (9./5.) + 32.
}

// converts fahrenheit to celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.) * 5./9.
}

// generates the nth fibonacci number
fn generate_nth_fib(n: usize) -> usize {
    // 1, 1, 2, 3, 5, 8
    if n == 0 {
        return 0;
    }
    
    let mut prev = 0;
    let mut number = 1;

    for _ in 1..n {
        let temp = number + prev;
        prev = number;
        number = temp; 
    }
    return number;
}

// prints out the lyrics of “The Twelve Days of Christmas” song
fn christmas_carol() {
    println!("TWELVE DAYS OF CHRISTMAS:");

    for day in 1..13 {
        day_intro(day);

        for gift_day in (1..(day + 1)).rev() {
            gift(
                gift_day,
                if gift_day == 1 && day != 1 {
                    "and "
                } else {
                    ""
                },
            );
        }
    }
}

fn day_intro(n: u32) {
    let day = match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };

    println!(
        "\nOn the {} day of Christmas\nmy true love sent to me:",
        day
    );
}

fn gift(n: u32, prefix: &str) {
    let gift_text = match n {
        1 => "a Partridge in a Pear Tree",
        2 => "Two Turtle Doves",
        3 => "Three French Hens",
        4 => "Four Calling Birds",
        5 => "Five Golden Rings",
        6 => "Six Geese a Laying",
        7 => "Seven Swans a Swimming",
        8 => "Eight Maids a Milking",
        9 => "Nine Ladies Dancing",
        10 => "Ten Lords a Leaping",
        11 => "Eleven Pipers Piping",
        12 => "12 Drummers Drumming",
        _ => "",
    };

    println!("{}{}", prefix, gift_text);
}