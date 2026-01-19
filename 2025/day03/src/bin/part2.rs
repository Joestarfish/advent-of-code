use std::env;
use std::fs;
use std::vec;

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

/**
 * I struggled with part 2 and was too focused on adapting part 1 instead of coming up with a
 * different solution. My initial plan was to take most of the logic from part 1 but remove the
 * lowest digit from the right before adding a new number. It was not the way to go.
 *
 * After a few tweaks and trying to come up with another algorithm, I ended up looking for a solution
 * to give me ideas on how this could be solved. I wanted to look at a solution in PHP so I would
 * not be able to do a 1:1 copy
 *
 * @see https://github.com/ancarofl/advent-of-code/blob/main/2025/day3/solution.php
 * */
fn find_largest_joltage(bank_to_parse: String) -> u64 {
    let mut batteries: Vec<u8> = vec![];
    let mut joltage_digits: [u8; 12] = [0; 12];

    for digit_char in bank_to_parse.chars() {
        let digit: u8 = match digit_char.to_string().parse() {
            Ok(num) => num,
            Err(_) => panic!("Could not parse digit to u8"),
        };
        batteries.push(digit);
    }

    let mut battery_start_index = 0;

    for nth_digit in 0..12 {
        // Compare the n next digits.
        // This will ensure we have at least 12 valid digits at the end of the loop
        let number_of_digits_to_compare = bank_to_parse.len() - (12 - nth_digit);

        let mut highest_digit = 0;

        // Take the largest of the n first digits
        for i in battery_start_index..=number_of_digits_to_compare {
            // take the largest number of the bunch
            if batteries[i] > highest_digit {
                highest_digit = batteries[i];
                battery_start_index = i + 1;
            }
        }

        joltage_digits[nth_digit] = highest_digit;
    }

    let stringify_joltage: String = joltage_digits.iter().map(ToString::to_string).collect();

    return match stringify_joltage.parse() {
        Ok(num) => num,
        Err(_) => panic!("Could not parse joltage to u64"),
    };
}
