use std::env;
use std::fs;

fn main() {
    // Parsing arguments
    let args: Vec<String> = env::args().collect();

    // We expect this program to be run with a file name as an argument
    if args.iter().count() != 2 {
        usage();
        return;
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .to_string();

    // Puzzle logic
    let mut sum_of_invalid_ids = 0;
    let ranges = contents.trim().split(",");

    for range in ranges {
        let mut first_and_last_num = range.split("-");

        let first_number: u64 = String::from(first_and_last_num.next().unwrap())
            .parse()
            .expect("First part was not a number");

        let last_number: u64 = String::from(first_and_last_num.next().unwrap())
            .parse()
            .expect("Last part was not a number");

        for i in first_number..=last_number {
            let stringify_number = i.to_string();

            // If the length is odd, we know it cannot repeat
            if stringify_number.len() % 2 != 0 {
                continue;
            }

            let middle = stringify_number.len() / 2;

            let number_parts = stringify_number.split_at(middle);

            if number_parts.0 == number_parts.1 {
                sum_of_invalid_ids += i;
            }
        }
    }

    println!("The sum is {sum_of_invalid_ids}");
}

fn usage() {
    println!("Usage: ./main <input file name>");
}
