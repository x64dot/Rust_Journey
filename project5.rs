/* For stdin/stdout needs */
use std::io;
use std::io::Write;

/* For path operations */
use std::path::Path;

/* related file operations module */
mod file_op {
    /* For file/path operations */
    use std::fs;
    use std::path::Path;

    pub fn read_file(path_to_file: &Path) -> String{
        let contents: String = match fs::read_to_string(path_to_file){
            Ok(contents) => contents,
            Err(error) => {
                panic!("Error reading file: {}", error);          
            },
        };
        return contents;
    }
}

/* related string operations since trim() decided to not work */
mod string_op{
    pub fn fully_trim(string: &String) -> String {
        return string.chars().filter(|c| !c.is_whitespace()).collect();
    }
    pub fn add_commas(string: &mut String) {
        *string = string.replace(".nse", ".nse,");
        if string.ends_with(",") {
            string.truncate(string.len() - 1);
        }
    }
}

fn main(){
    println!("-------------------------------");
    println!("x64Dot nmap script organizer");
    println!("-------------------------------");

    let mut file_path: String = String::new();

    print!("Enter file path: ");

    match io::stdout().flush() {
        Err(error) => eprintln!("Error regarding flushing stdout buffer: {}", error),
        Ok(_) => print!(""),
    };

    match io::stdin().read_line(&mut file_path){
        Ok(_) => {
            file_path = file_path.trim().to_string();
        },

        Err(error) => {
            panic!("Error reading input: {}", error);
        },
    };
    println!("File Path => {}\n", file_path);

    /* To convert String to Path */
    let file_realpath: &Path = Path::new(&file_path);

    let mut contents = file_op::read_file(&file_realpath);
   
   
    contents = string_op::fully_trim(&contents);
    string_op::add_commas(&mut contents);
   
    println!("Example nmap command: nmap -sV --script={} (IP)", contents); 
}
