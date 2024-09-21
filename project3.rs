use std::io;
use std::process;

fn main(){
  let mut length_string = String::new();
  let mut width_string = String::new();
 
  println!("Enter length of room(in feet): ");
  let _ = io::stdin().read_line(&mut length_string);

  if length_string.trim().is_empty() == true {
    println!("Input was empty. Please rerun.");
    process::exit(1);
  }

  println!("Enter width of room(in feet): ");
  let _ = io::stdin().read_line(&mut width_string);
 
  if width_string.trim().is_empty() == true {
    println!("Input was empty. Please rerun.");
    process::exit(1);
  }

  let length_int = length_string.trim().parse::<i32>().expect("Invalid length input");
  
  let width_int = width_string.trim().parse::<i32>().expect("Invalid width input");
 
  let area: i32 = get_area(length_int, width_int);
 
  println!("Area in square foot is: {}", area);
 
}

fn get_area(length: i32, width: i32) -> i32{
  return length * width;
}
