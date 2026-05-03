mod board;

use std::fs::write;
use std::fs::read_to_string;
use board::*;

fn main() {
    let command_line: Vec<String> = std::env::args().collect();

    if command_line.len() < 4 {
        eprintln!("Missing command line arguments");
        return;
    }

    let file_name = &command_line[1];
    let command = &command_line[2];
    let params = &command_line[3];

    match command.as_str() {
        "new" => {
            let parts: Vec<&str> = params.split(",").collect();

            if parts.len() != 4 {
                eprintln!("Expected 4 boat counts, example: 4,3,2,1");
                return;
            }

            let boats: Vec<u8> = parts
                .iter()
                .map(|s| s.parse::<u8>())
                .filter_map(Result::ok)
                .collect();

            if boats.len() != 4 {
                eprintln!("Invalid boat counts");
                return;
            }

            let board = Board::new(&boats);

            match write(file_name, board.to_string()) {
                Ok(()) => (),
                Err(e) => eprintln!("{}", e),
            }
        }

        "add_boat" => {
            let content = match read_to_string(file_name) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };

            let board = Board::from(content);

            let parts: Vec<&str> = params.split(",").collect();

            if parts.len() != 4 {
                eprintln!("Expected: V,3,10,10 or H,3,10,10");
                return;
            }

            let len = match parts[1].parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Invalid boat length");
                    return;
                }
            };

            let row = match parts[2].parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Invalid row");
                    return;
                }
            };

            let col = match parts[3].parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Invalid column");
                    return;
                }
            };

            let boat = match parts[0] {
                "V" => Boat::Vertical(len),
                "H" => Boat::Horizontal(len),
                _ => {
                    eprintln!("Invalid direction");
                    return;
                }
            };

            match board.add_boat(boat, (row, col)) {
                Ok(new_board) => {
                    match write(file_name, new_board.to_string()) {
                        Ok(()) => (),
                        Err(e) => eprintln!("{}", e),
                    }
                }
                Err(_) => {
                    eprintln!("Cannot add boat");
                }
            }
        }

        _ => {
            eprintln!("Unknown command");
        }
    }
}