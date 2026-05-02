const BSIZE : usize = 20;
pub struct Board {
    boats : [u8; 4] ,
    data : [[ u8; BSIZE]; BSIZE] ,
}
pub enum Error { Overlap , OutOfBounds , BoatCount }
pub enum Boat { Vertical ( usize ) , Horizontal ( usize ) }

impl Board {
    /// Crea una board vuota con la disponibilita ’ di navi specificata .
    pub fn new ( boats : &[ u8 ]) -> Board { todo !() }
    /// Crea una Board a partire dal contenuto del file ( come stringa ).
    pub fn from ( s : String ) -> Board { todo !() }
    /// Aggiunge la nave ; restituisce la nuova Board o un errore .
    pub fn add_boat (self , boat : Boat , pos : (usize , usize ) ) -> Result < Board , Error > {
        todo !()
    }
    /// Converte la board in una stringa salvabile su file .
    pub fn to_string (& self ) -> String { todo !() }
}