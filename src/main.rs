/// is_choice_1 will give a user two choices, read their input from stdin, and return 
/// true if they chose `choice1`, and false if `choice2`.
fn is_choice_1(choice1: &str, choice2: &str) -> bool {
    print!("{} (1) -- {} (2)?", choice1, choice2);
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => match input.as_str() {
            "1" => true,
            "2" => false,
            _ => {
                println!("Your input {} was not 1 or 2. Please try again", input);
                is_choice_1(choice1, choice2)
            }
        }
        Err(_) => {
            println!("Could not read your input. Please try again");
            is_choice_1(choice1, choice2)
        }
    }
}

fn main() {
    println!("Hello, world!");
}
