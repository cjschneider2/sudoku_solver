pub struct Board {
    entries: [[usize; 9]; 9]
}

impl Board {
    /// Checks to see if the board is a valid sudoku solution.
    /// Three "rules" need to be checked:
    ///     column unique: Each column has to have the #'s 1..9 w/o repeats
    ///     row unique: Each row has to have the #'s 1..9 w/o repeats
    ///     zone unique: Each 3x3 area needs to have the #'s 1..9 w/o repeats
    ///
    /// NOTE: This solution currently rejects partially filled out boards.
    pub fn is_valid_solution(&self) -> bool {
        // Check columns
        for col in 0..9 {
            let check = Board::check_column(&self, col);
            match check {
                None => return false,
                Some(remaining) => { if remaining.len() > 0 { return false; }
                    //println!("Remaining in column: {:?}", remaining)
                }
            }
        }
        // Check rows
        for row in 0..9 {
            let check = Board::check_row(&self, row);
            match check {
                None => return false,
                Some(remaining) => { if remaining.len() > 0 { return false; }
                    //println!("Remaining in row: {:?}", remaining)
                }
            }
        }
        // Check zones
        for zone in 0..9 {
            let check = Board::check_zone(&self, zone);
            match check {
                None => return false,
                Some(remaining) => { if remaining.len() > 0 { return false; }
                    //println!("Remaining in zone: {:?}", remaining)
                }
            }
        }
        // Return the default case if all the other checks have been valid.
        true
    }

    fn check_column(&self, col_num: usize) -> Option<Vec<usize>> {
        // Create a new return vector which will contain the numbers which
        // are still valid for this column.
        let mut return_vector:Vec<usize> = Vec::new();
        // we'll check every (num)ber to see if it is in the column, if it
        // is we'll mark it and push it onto the return_vector.
        for num in 1..10 { // [1,10) in rust
            let mut contains_num = false;
            for idx in 0..9 {
                if self.entries[idx][col_num] == num {
                    // we can't have double numbers so we'll check here for it
                    // and return a None value in that case.
                    if contains_num {
                        return None;
                    } else {
                        contains_num = true;
                    }
                }
            }
            if !contains_num {
                return_vector.push(num);
            }
        }
        Some(return_vector)
    }

    fn check_row(&self, row_num: usize) -> Option<Vec<usize>> {
        let mut return_vector:Vec<usize> = Vec::new();
        for num in 1..10 { // [1,10) in rust
            let mut contains_num = false;
            for idx in 0..9 {
                if self.entries[row_num][idx] == num {
                    if contains_num {
                        return None;
                    } else {
                        contains_num = true;
                    }
                }
            }
            if !contains_num {
                return_vector.push(num);
            }
        }
        Some(return_vector)
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
        let mut return_vector:Vec<usize> = Vec::new();
        let coord_iter = self.zone_range(zone_num).unwrap_or(Vec::new());
        for num in 1..10 { // NOTE: the range [1,9] in rust
            let mut contains_num = false;
            for coord in &coord_iter {
                match *coord {
                    (idx,idy) => {
                        if self.entries[idx][idy] == num {
                            if contains_num {
                                return None;
                            } else {
                                contains_num = true;
                            }
                        }
                    }
                }
            }
            if !contains_num {
                return_vector.push(num);
            }
        }
        Some(return_vector)
    }

    /// This is a helper function to calculate the ranges needed and returns
    /// a vector of tuple coordinates that can be iterated upon.
    fn zone_range(&self, zone_num: usize) -> Option<Vec<(usize,usize)>> {
        // A simple check to see if we are staying within the bounds...
        // Probably not needed if it's called correctly.
        if zone_num > 8 { return None; }
        let mut iter_vec:Vec<(usize,usize)> = Vec::new();
        // Calculate the starting row and column for the zone
        let col = (zone_num % 3) * 3;
        let row = ((zone_num as f32 / 3f32).floor() as usize)*3;
        // we'll fill the vector we created with the coordinates of the zone.
        for idx in row..(row+3) {
            for idy in col..(col+3) {
                iter_vec.push((idx,idy));
            }
        }
        return Some(iter_vec)
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
