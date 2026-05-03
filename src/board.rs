const BSIZE: usize = 20;

pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}

pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}

pub enum Boat {
    Vertical(usize),
    Horizontal(usize),
}

impl Board {
    pub fn new(boats: &[u8]) -> Board {
        let mut b = [0u8; 4];

        for i in 0..4 {
            b[i] = boats[i];
        }

        Board {
            boats: b,
            data: [[0u8; BSIZE]; BSIZE],
        }
    }
    pub fn from(s: String) -> Board {
        let mut lines = s.lines();

        let first_line = lines.next().unwrap();
        let nums: Vec<u8> = first_line
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        let mut boats = [0u8; 4];
        for i in 0..4 {
            boats[i] = nums[i];
        }

        let mut data = [[0u8; BSIZE]; BSIZE];

        for (r, line) in lines.enumerate() {
            for (c, ch) in line.chars().enumerate() {
                if ch == 'B' {
                    data[r][c] = 1;
                }
            }
        }

        Board { boats, data }
    }
    pub fn add_boat(self, boat: Boat, pos: (usize, usize)) -> Result<Board, Error> {
        let mut new_board = self;

        let row = pos.0 - 1;
        let col = pos.1 - 1;

        let len = match boat {
            Boat::Vertical(n) => n,
            Boat::Horizontal(n) => n,
        };

        if len < 1 || len > 4 {
            return Err(Error::BoatCount);
        }

        if new_board.boats[len - 1] == 0 {
            return Err(Error::BoatCount);
        }

        let positions: Vec<(usize, usize)> = match boat {
            Boat::Vertical(n) => {
                if row + n > BSIZE {
                    return Err(Error::OutOfBounds);
                }

                (0..n).map(|i| (row + i, col)).collect()
            }

            Boat::Horizontal(n) => {
                if col + n > BSIZE {
                    return Err(Error::OutOfBounds);
                }

                (0..n).map(|i| (row, col + i)).collect()
            }
        };

        for &(r, c) in &positions {
            let r_start = if r == 0 { 0 } else { r - 1 };
            let r_end = if r + 1 >= BSIZE { BSIZE - 1 } else { r + 1 };

            let c_start = if c == 0 { 0 } else { c - 1 };
            let c_end = if c + 1 >= BSIZE { BSIZE - 1 } else { c + 1 };

            for rr in r_start..=r_end {
                for cc in c_start..=c_end {
                    if new_board.data[rr][cc] == 1 {
                        return Err(Error::Overlap);
                    }
                }
            }
        }

        for (r, c) in positions {
            new_board.data[r][c] = 1;
        }

        new_board.boats[len - 1] -= 1;

        Ok(new_board)
    }
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str(&format!(
            "{} {} {} {}\n",
            self.boats[0], self.boats[1], self.boats[2], self.boats[3]
        ));

        for r in 0..BSIZE {
            for c in 0..BSIZE {
                if self.data[r][c] == 1 {
                    result.push('B');
                } else {
                    result.push(' ');
                }
            }
            result.push('\n');
        }
        result
    }
}