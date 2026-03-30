use std::io::{self, Write};


fn read_input() -> String {
    print!(">");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    /*return*/ input.trim().to_string()
}


fn main() {
    let mut location = "House"; 

    loop {
        match location.as_ref() {
            "House" => {
                println!("You are at home, what would you like to do?");
                println!("open door, go upstairs");

                //Matching The Input
                match read_input().as_str() {
                    "open door" => println!("You open the door"),
                    "go upstairs" => { 
                        println!("You go upstairs"); 
                        location = "Upstairs"; 
                    }
                    _ => println!("That is not a command"),
                }
            }
            "Upstairs" => {
                println!("There is nothing upstairs.");
                println!("go downstairs");

                match read_input.as_str() {
                    "go downstairs" => { 
                        println!("You go downstairs"); 
                        location = "House"; 
                    }
                    _ => println!("That is not a command"),
                }
            }
            _ => println!("idk"),
        }
    }
}
