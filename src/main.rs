use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        guessing_game();
    
        println!("do ya wanna get wreckt again? (y/n)");
        let mut yesnt = String::new();
        io::stdin()
            .read_line(&mut yesnt)
            .expect("failure");

        if yesnt.trim().to_lowercase() != "y"{
           println!("\n thankyou for playing");
           break;
    }
}
}

fn guessing_game() {
    println!("guess the number!");
    let mut total_count = 0;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess: ");
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
        
        let distance = input.abs_diff(secret_number);


        match input.cmp(&secret_number) {
            Ordering::Less => {
                println!("too small!");

            if distance <= 3 {
                println!("very close vro");
            }else if distance <= 10 {
                println!("close vro");
            }else {
                println!("too far vro");
            }
        }

            Ordering::Greater => {
                println!("too big!");

           if distance <= 3 {
               println!("very close vro");
           }else if distance <= 10{
               println!("close vro");
           }else{
               println!("too far vro");
           }
        }

            Ordering::Equal => { 
                println!("You win, your total count was {}", total_count);
                break;
            }
        }
    }
}
