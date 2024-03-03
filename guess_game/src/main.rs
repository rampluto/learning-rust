use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");
    
    let _secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please input your guess!");
        
        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&_secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }

}
