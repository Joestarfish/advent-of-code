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

            // This is an int since we divide with 2.
            // We would have to make a float division to get a float result.
            // Here it is what we want so no extra steps needed
            let middle = stringify_number.len() / 2;

            /*
             * 111111 = 6 * "1"
             * 121212 = 3 * "12"
             * 123123 = 2 * "123"
             */

            for number_of_chars in 1..=middle {
                let mut slice_of_number = stringify_number.clone();
                let _ = slice_of_number.split_off(number_of_chars);

                let repeat_times = stringify_number.len() / number_of_chars;

                if slice_of_number.repeat(repeat_times) == stringify_number {
                    sum_of_invalid_ids += i;
                    break;
                }
            }
        }
    }

    println!("The sum is {sum_of_invalid_ids}");
}

fn usage() {
    println!("Usage: ./main <input file name>");
}
