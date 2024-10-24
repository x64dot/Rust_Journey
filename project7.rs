use std::thread;
use std::sync::{Arc, Mutex};

fn main(){
    let mut handles: Vec<_> = vec![];
    let counter = Arc::new(Mutex::new(0));


    for _ in 1..=10 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move ||{
            let inc: i32 = 1;

            let mut data = counter_clone.lock().unwrap();

            *data += inc;
            println!("{}", *data);
        });

        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Total value: {:?}", counter);

}
