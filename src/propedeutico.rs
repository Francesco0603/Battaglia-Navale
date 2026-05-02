use std::time::SystemTime;

pub enum MyError {
    Simple ( SystemTime ) ,
    Complex ( SystemTime , String ) ,
}

pub enum MulErr { Overflow , NegativeNumber }
pub fn mul ( a : i32 , b : i32) -> Result <u32 , MulErr > {
    if a < 0 && b >= 0 || b < 0 && a >= 0 {return Err(MulErr::NegativeNumber)}
    match a.checked_mul(b){
        Some(res) => Ok(res as u32),
        None => Err(MulErr::Overflow),
    }
}

pub fn print_error(e: MyError) {

    match e {

        MyError::Simple(time) => {

            println!("Errore semplice");
            println!("Tempo: {:?}", time);
        }

        MyError::Complex(time, s) => {

            println!("Errore complesso");

            println!("Tempo: {:?}", time);

            println!("Messaggio: {}", s);
        }
    }
}
