// Bringing libraries into scope
use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main(){
    println!("Welcome to the guesssing game");

    // Create a random number between 1 and 100 using rand crate
    let random_number = rand::thread_rng().gen_range(1..101);
    
    print!("Random number is: {} \n", random_number);

    // Create a loop to keep asking for input until they get it correct
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}",guess) ;

        // Convert the guess to a number (shadowing the guess variable)
        let guess:u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("{}","Please type a number".red());
                continue;
            },
        };

        // Compare the guess to the random number
        match guess.cmp(&random_number){
            Ordering::Less=>println!("{}","Too small".red()),
            Ordering::Greater=>println!("{}","Too big".red()),
            Ordering::Equal=>{
                println!("{}","You win".green());
                break;
            }
        }
    }

}
