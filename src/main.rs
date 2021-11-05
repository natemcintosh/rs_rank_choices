use std::{collections::HashMap, io::Write};

use clap::{App, Arg};
use itertools::Itertools;

/// is_choice_1 will give a user two choices, read their input from stdin, and return
/// true if they chose `choice1`, and false if `choice2`.
fn is_choice_1(choice1: &str, choice2: &str) -> bool {
    print!("{} (1) -- {} (2)?  ", choice1, choice2);
    let _ = std::io::stdout().flush();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => match input.as_str().trim() {
            "1" => true,
            "2" => false,
            _ => {
                println!("Your input '{}' was not 1 or 2. Please try again", input);
                is_choice_1(choice1, choice2)
            }
        },
        Err(_) => {
            println!("Could not read your input. Please try again");
            is_choice_1(choice1, choice2)
        }
    }
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn nchoosek(n: usize, k: usize) -> usize {
    factorial(n) / (factorial(k) * factorial(n - k))
}

fn main() {
    let matches = App::new("Rank Choices")
        .version("0.1")
        .author("Nathan McIntosh")
        .about("Helps you rank choices that you have in a text file")
        .arg(
            Arg::with_name("text_file")
                .help("The path to a text file containing all options. Each option should be on a new line.")
                .required(true)
                .takes_value(true))
        .get_matches();

    // Read the input file into a Vec<&str>
    let options = std::fs::read_to_string(
        matches
            .value_of("text_file")
            .expect("Did not get a path to file of options."),
    )
    .unwrap();
    let options: Vec<&str> = options.lines().collect();

    // Create a HashMap to store the results in
    let mut results: HashMap<&str, usize> = HashMap::new();

    // Tell the user how many comparisons they'll have to do
    let n_iterations = nchoosek(options.len(), 2);
    println!(
        "You will have {} comparisons to do. Press 'control + C' if you do not wish to continue",
        n_iterations
    );

    // Loop over all the combinations of size 2, and ask the user to compare them
    for (idx, (&choice1, &choice2)) in options.iter().tuple_combinations().enumerate() {
        print!("{}/{}: ", idx + 1, n_iterations);
        let choice = is_choice_1(choice1, choice2);
        match choice {
            true => {
                let counter = results.entry(choice1).or_insert(0);
                *counter += 1;
            }
            false => {
                let counter = results.entry(choice2).or_insert(0);
                *counter += 1;
            }
        }
    }

    println!("\n\nThe final results were");
    results
        .iter()
        .sorted_by(|(_, &count1), (_, &count2)| Ord::cmp(&count2, &count1))
        .for_each(|(&choice, &count)| println!("{} -- {}", choice, count));
}
