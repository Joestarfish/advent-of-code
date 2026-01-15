use std::env;
use std::fs;

const MAX_DIAL_POSITION: u8 = 99;

struct ParsedRotation {
    rotation_side: String,
    number_of_clicks: u16,
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
    let mut current_dial_position: u8 = 50;
    let mut remaining_rotations;
    let mut number_of_zeros = 0;

    for line in contents.lines() {
        let parsed_rotation = parse_rotation(line.to_string());

        remaining_rotations = parsed_rotation.number_of_clicks;

        while remaining_rotations > 0 {
            remaining_rotations -= 1;
            current_dial_position = do_one_rotation(&parsed_rotation, current_dial_position);
        }

        if current_dial_position == 0 {
            number_of_zeros += 1;
        }

        println!("{} -> {}", line, current_dial_position);
    }

    println!("We got {number_of_zeros} zeros");
}

fn usage() {
    println!("Usage: ./main <input file name>");
}

fn parse_rotation(line_to_parse: String) -> ParsedRotation {
    let mut chars = line_to_parse.chars();
    let rotation_side = String::from(chars.next().unwrap());

    let number_of_clicks: u16 = match chars.as_str().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    ParsedRotation {
        rotation_side: rotation_side,
        number_of_clicks: number_of_clicks,
    }
}

fn do_one_rotation(parsed_rotation: &ParsedRotation, current_dial_position: u8) -> u8 {
    if parsed_rotation.rotation_side == "L" {
        if current_dial_position == 0 {
            return MAX_DIAL_POSITION;
        } else {
            return current_dial_position - 1;
        }
    }

    if current_dial_position == MAX_DIAL_POSITION {
        0
    } else {
        current_dial_position + 1
    }
}

