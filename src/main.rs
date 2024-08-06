use std::io;

fn main() {
    println!("Welcome to a Fibonacci generator!");
    println!("Please enter how many numbers do you need:");

    let mut nth_row: String;
    let nth: u16;

    loop {
        nth_row = String::from("");

        io::stdin().
            read_line(&mut nth_row).
            expect("Can't read line");

        nth = match nth_row.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Value isn't valid.");
                println!("Please enter a number again:");
                continue;
            }
        };

        break;
    }

    let mut index: u16 = 0;
    let mut fib_prev: u32 = 0;
    let mut fib_current: u32 = 1;

    println!("Fibonacci numbers:");

    while index < nth {
        println!("{}", fib_prev);

        fib_current += fib_prev;
        fib_prev = fib_current - fib_prev;
        index += 1;
    }
}
