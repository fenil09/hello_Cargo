
use std::io::{self, Stdin};
use rand::Rng;
use std::cmp::Ordering; // using this enum to perform comparisions between two things in rust.
fn main() {
    println!("Guess the number Game");
    let secret_number  = rand::thread_rng().gen_range((1), 101);
    print!("secret_number is:{}",secret_number);
    
    loop {
      println!("Please input your guess number");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to readlinge");
        print!("You guessed:{}",guess);
        // we would be using rand to basically generate the random number
        // Now one issue we would face is we are taking string from the user and then
        // comparing it with an integer so we need to change that.
        let intgues: u32 = guess.trim().parse().expect("Please enter a number");
   
        // So In rust if we want to compare two expressions then we have to use the match keyword
   
        match intgues.cmp(&secret_number) {
          Ordering::Less => print!("Too small number "),
          Ordering::Equal => {
            print!("Yes it is correct great job");
            break;
          },
          Ordering::Greater => print!("You guessed a bit higher"),
        }
   
    }
    // storing value we need a variable which we create using let keyword
    

}
