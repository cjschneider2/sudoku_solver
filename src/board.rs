pub struct Board {
    entries: [[usize; 9]; 9]
}

impl Board {
    /// Checks to see if the board is a valid sudoku solution.
    /// Three "rules" need to be checked:
    ///     column unique: Each column has to have the #'s 1..9 w/o repeats
    ///     row unique: Each row has to have the #'s 1..9 w/o repeats
    ///     zone unique: Each 3x3 area needs to have the #'s 1..9 w/o repeats
    pub fn is_valid_solution(&self) -> bool {
        // Check columns
        for col in 0..9 {
            let check = Board::check_column(&self, col);
            match check {
                None => (),
                Some(remaining) => {
                    println!("The following numbers are remaining: {:?}",
                             remaining)
                }
            }
        }
        // Check rows
        // Check zones
        true
    }

    fn check_column(&self, col_num: usize) -> Option<Vec<usize>> {
        Some(vec!{1,2,3})
    }

    fn check_row(&self, row_num: usize) -> Option<Vec<usize>> {
        None
    }

    /// The zones are numbered 0..8 in this order:
    ///     +-+-+-+
    ///     |0|1|2|
    ///     +-+-+-+
    ///     |3|4|5|
    ///     +-+-+-+
    ///     |6|7|8|
    ///     +-+-+-+
    fn check_zone(&self, zone_num: usize) -> Option<Vec<usize>> {
        None
    }

    /// Displays the board in a wonderfully retro ASCII style.
    pub fn display(&self) {
        println!("+---+---+---+");
        for i in 0..3 {
            print!("|");
            for j in 0..3 {
                print!("{}", self.entries[i][j]);
            }
            print!("|");
            for j in 3..6 {
                print!("{}", self.entries[i][j]);
            }
            print!("|");
            for j in 6..9 {
                print!("{}", self.entries[i][j]);
            }
            println!("|");
        }
        println!("+---+---+---+");
        for i in 3..6 {
            print!("|");
            for j in 0..3 {
                print!("{}", self.entries[i][j]);
            }
            print!("|");
            for j in 3..6 {
                print!("{}", self.entries[i][j]);
            }
            print!("|");
            for j in 6..9 {
                print!("{}", self.entries[i][j]);
            }
            println!("|");
        }
        println!("+---+---+---+");
        for i in 6..9 {
            print!("|");
            for j in 0..3 {
                print!("{}", self.entries[i][j]);
            }
            print!("|");
            for j in 3..6 {
                print!("{}", self.entries[i][j]);
            }
            print!("|");
            for j in 6..9 {
                print!("{}", self.entries[i][j]);
            }
            println!("|");
        }
        println!("+---+---+---+");
    }
}

pub fn new_empty( ) -> Board {
    Board { entries:
       [[ 0,0,0, 0,0,0, 0,0,0],
        [ 0,0,0, 0,0,0, 0,0,0],
        [ 0,0,0, 0,0,0, 0,0,0],

        [ 0,0,0, 0,0,0, 0,0,0],
        [ 0,0,0, 0,0,0, 0,0,0],
        [ 0,0,0, 0,0,0, 0,0,0],

        [ 0,0,0, 0,0,0, 0,0,0],
        [ 0,0,0, 0,0,0, 0,0,0],
        [ 0,0,0, 0,0,0, 0,0,0]] }
}

pub fn new_with_entries( input_entries: [[usize; 9]; 9] ) -> Board {
    Board { entries: input_entries }
}
