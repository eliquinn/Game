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

    println!("Enter your name: ");

    let mut name = &read_input();

    let mut money: u128 = 0;

    loop {
        match location.as_ref() {
            "House" => {
                println!("You are at home, what would you like to do?");
                println!("Options: open door, go upstairs");

                //Matching The Input
                match read_input().as_str() {
                    "open door" => {
                        println!("You open the door");
                        location = "Outside";
                    }
                    "go upstairs" => { 
                        println!("You walk upstairs."); 
                        location = "Upstairs"; 
                    }
                    "fard" => {
                        location = "fard";
                    }
                    _ => println!("That is not a command"),
                }
            }

            "Upstairs" => {
                println!("There is nothing upstairs.");
                println!("Options: go downstairs");

                match read_input().as_str() {
                    "go downstairs" => { 
                        println!("You go downstairs"); 
                        location = "House"; 
                    }
                    _ => println!("That is not a command"),
                }
            }

            "Outside" => {
                println!("You are outside.");
                println!("Options; go home, talk to noob");
                println!("You have {money} money");
                
                match read_input().as_str() {
                    "go home" => { 
                        println!("You go to your home sweet home"); 
                        location = "House"; 
                    }
                    "talk to noob" => {
                        println!("The noob says: \"Here is free money!\"");
                        money += 1;
                    }
                    _ => println!("That is not a command"),
                }
            }
            _ => { 
                eprintln!("ERROR: Location does not exist. Quitting.");
                std::process::exit(1);
            }
        }
    }
}
