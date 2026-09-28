use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    guessing_game();
}

fn guessing_game() {
    println!("guess the number!");
    let mut total_count = 0;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess.");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please only enter numbers vro");
                continue;
            }
        };

        println!("you guessed: {}", input);
        total_count += 1;
       

        match input.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!!, your total count was {}", total_count);
                break;
            }
        }
    }
}
