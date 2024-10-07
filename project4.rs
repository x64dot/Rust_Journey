use std::io;
use std::io::Write;
use std::process; /* To use process::exit */
use std::fs::File;
use std::path::Path;
use std::ffi::OsString;

fn main(){
    let mut option1: String = String::new();
    let mut save_to_file: bool = false;
    let mut min_string: String = String::new();
    let mut max_string: String = String::new();

    println!("Welcome to x64Dot number generator");

    print!("Would you like to save the numbers in a file? (y/n): ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut option1).expect("Failed to read input.");

    if option1.trim().to_lowercase() == "y" {
        save_to_file = true;
    }
    else if option1.trim().to_lowercase() == "n" {
        save_to_file = false;
    }
    else {
        println!("Invalid input, please rerun the program to try again.");
        
        process::exit(0);
    }

    if save_to_file {
        let mut file_name: String = String::new();
        print!("Enter min number to generate from: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut min_string).expect("Failed to read input");

        print!("Enter max number to generate to: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut max_string).expect("Failed to read input.");
        
        print!("Enter File name: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut file_name).expect("Failed to read input.");


        let min: i32 = min_string.trim().parse().unwrap();
        let max: i32 = max_string.trim().parse().unwrap();
        
        let filename_OsString: OsString = file_name.trim().into();
        let path = Path::new(&filename_OsString); 
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create file: {}", why),
            Ok(file) => file,
        };

        for number in min..max + 1 {
            let mut number_string: String = number.to_string();
            number_string.push_str("\n");

            let _ = file.write_all(number_string.as_bytes());
        }
        println!("Generated numbers saved to {} file.", display);
        println!("Thank you for using x64dot program!");
        
    }
    else {
        print!("Enter min number to generate from: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut min_string).expect("Failed to read input");

        print!("Enter max number to generate to: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut max_string).expect("Failed to read input.");

        let min: i32 = min_string.trim().parse().unwrap();
        let max: i32 = max_string.trim().parse().unwrap();

        for number in min..max + 1 {
            println!("{}", number);
        }
        println!("The numbers was generated! Thank you for using x64dot Program!");

    }
}
