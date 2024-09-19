use std::io;
use rand::Rng; 

fn menu(){
    println!("Welcome to my number guessing game.");
    println!("The number is between 1-10. You have five attempts!");
}

fn main(){
    menu();
    let mut attempts: i32 = 0;
    let mut input: String; 
    
    let rng = rand::thread_rng().gen_range(1..10);
    let num_str = rng.to_string();
    /* Uncomment to see the generated number.
    println!("Number: {}", num_str); 
    */
    loop {
        input = String::new();

        println!("Enter a number >> ");
        let _ = io::stdin().read_line(&mut input);

        let trimmed_input = input.trim();

        if trimmed_input == num_str {
            println!("You guessed it!");
            println!("You took a total of {} attempts.", attempts);

            break;
        }

        if attempts >= 5 {
            println!("Sorry, but you used all of your 5 attempts.");
            break;
        }
        attempts += 1;
    }
 
}
