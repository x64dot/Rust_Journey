struct Under_Main {
    x: Option<i32>,
}

struct Main {
    num_one: Under_Main,
    num_two: Under_Main,
}

fn main(){
    let mut main_struct: Main = Main {
        num_one: Under_Main {x: None},
        num_two: Under_Main {x: None},
    };
    
    main_struct.num_one.x = Some(5);
    main_struct.num_two.x = Some(5);
    
    let num1 = match main_struct.num_one.x {
        Some(value) => value,
        None => 0,
    };
    
    println!("Num1: {}", num1);
    
    let num2 = match main_struct.num_two.x{
        Some(value) => value,
        None => 0,
    };
    println!("Num2: {}", num2);
    
    println!("Sum: {}", num1 + num2);
}
