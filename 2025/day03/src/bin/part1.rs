use std::env;
use std::fs;

#[derive(Debug)]
struct HighestBetteryState {
    highest_number: u8,
    sum: u16,
}

fn main() {
    // Parsing arguments
    let args: Vec<String> = env::args().collect();

    // We expect this program to be run with a file name as an argument
    if args.iter().count() != 2 {
        usage();
        return;
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Puzzle logic
    let mut total_sum = 0;

    for bank in contents.lines() {
        total_sum += find_largest_joltage(bank.to_string());
    }

    println!("The sum is {total_sum}");
}

fn usage() {
    println!("Usage: ./main <input file name>");
}

fn find_largest_joltage(bank_to_parse: String) -> u16 {
    let mut state = HighestBetteryState {
        highest_number: 0,
        sum: 0,
    };

    for battery in bank_to_parse.chars() {
        let current_number: u8 = match battery.to_string().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        let current_sum: u16 = (state.highest_number * 10 + current_number).into();

        if current_sum > state.sum {
            state.sum = current_sum;
        }

        if current_number > state.highest_number {
            state.highest_number = current_number;
        }
    }

    state.sum
}
