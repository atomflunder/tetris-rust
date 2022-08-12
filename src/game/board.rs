use std::fmt;

#[derive(Clone)]
pub struct Board {
    // This is to look if the coordinates are populated,
    // '#' if a piece is there, ' ' if not.
    pub board: [[char; 10]; 20],
    // This stores the color information of the coordinates separately,
    // empty is White, else it's the piece's color.
    pub color: [[(u8, u8, u8); 10]; 20],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.board {
            for y in x {
                if y == ' ' {
                    write!(f, "-")?;
                } else {
                    write!(f, "{}", y)?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[' '; 10]; 20],
            color: [[(255, 255, 255); 10]; 20],
        }
    }
}
