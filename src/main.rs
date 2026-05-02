mod board;
mod propedeutico;

use std::time::SystemTime ;
use std::fs::write;
use std::fs::read_to_string;
use propedeutico::*;
use board::*;

fn main(){
    let board = read_to_string("src/board.txt");
    let done = write("src/board.txt","tentativo scrittura2");
    match board {
        Ok(content) => println!("{}", content),
        Err(e) => {
            let mio_errore = MyError::Complex(SystemTime::now(),e.to_string());
            print_error(mio_errore);
        }
    }
    match done {
        Ok(()) => (),
        Err(e) => {print_error(MyError::Complex(SystemTime::now(),e.to_string()))}
    }
}