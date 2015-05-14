use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Board {
    entries: [[usize; 9]; 9]
}

impl Board {

    pub fn val_at(&self, x: usize, y: usize) -> usize {
        self.entries[x][y]
    }

    pub fn insert(&mut self, value: usize, x: usize, y: usize) {
        self.entries[x][y] = value;
    }

    /// Checks to see if the board is a valid sudoku solution.
    /// Three "rules" need to be checked:
    ///     column unique: Each column has to have the #'s 1..9 w/o repeats
    ///     row unique: Each row has to have the #'s 1..9 w/o repeats
    ///     zone unique: Each 3x3 area needs to have the #'s 1..9 w/o repeats
    ///
    /// Returns: A bool tuple in the form of (valid, complete)
    pub fn is_valid_solution(&self) -> (bool, bool) {
        let mut valid = true;
        let mut complete = true;
        // Check columns
        println!("Checking columns");
        for col in 0..9 {
            let check = Board::check_column(&self, col);
            match check {
                None => valid = false,
                Some(remaining) => {
                    if remaining.len() > 0 { complete = false; }
                }
            }
        }
        // Check rows
        println!("Checking rows");
        for row in 0..9 {
            let check = Board::check_row(&self, row);
            match check {
                None => valid =  false,
                Some(remaining) => {
                    if remaining.len() > 0 { complete = false; }
                }
            }
        }
        // Check zones
        println!("Checking zones");
        for zone in 0..9 {
            let check = Board::check_zone(&self, zone);
            match check {
                None => valid = false,
                Some(remaining) => {
                    if remaining.len() > 0 { complete = false; }
                }
            }
        }
        // Return the default case if all the other checks have been valid.
        println!("Returning: ({},{})", valid, complete);
        (valid, complete)
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
        Some(iter_vec)
    }

    /// Finds and returns a vector of valid posibilites for the given coordinates.
    pub fn get_valid_pos(&self, x:usize, y:usize) -> Vec<usize> {
        let mut result_vec:Vec<usize> = Vec::new();
        let x_z = (x as f32/3f32).floor() as usize;
        let y_z = (y as f32/3f32).floor() as usize;
        let zone_num = x_z*3 + y_z;
        // We can check the col, row, and zone to get a set of posibilities.
        let row = self.check_row(x).unwrap_or(Vec::new());
        let col = self.check_column(y).unwrap_or(Vec::new());
        let zone = self.check_zone(zone_num).unwrap_or(Vec::new());

        // Here comes the problem hoever, we can't just merge and dedup the list.
        // We have to make sure that each possible entry is also valid for the others
        // otherwise it's not a valid move.
        let mut map:HashMap<usize,usize> = HashMap::with_capacity(9);
        for itr in row {
            let is_in = *map.get_mut(&itr).unwrap_or(&mut 0);
            map.insert(itr, is_in+1);
        }
        for itr in col {
            let is_in = *map.get_mut(&itr).unwrap_or(&mut 0);
            map.insert(itr, is_in+1);
        }
        for itr in zone{
            let is_in = *map.get_mut(&itr).unwrap_or(&mut 0);
            map.insert(itr, is_in+1);
        }
        // We'll sort out the entries with more than one appearance into the
        // result vector
        for (key, val) in map.iter() {
            if *val > 1 { result_vec.push(*key); }
        }
        result_vec.sort();
        result_vec
    }

    /// Finds the next empty location
    pub fn next_empty(&self) -> Option<(usize,usize)> {
        let (mut e_x, mut e_y, mut empty) = (0,0, true);
        for idx in 0..9 {
            for idy in 0..9 {
                if self.entries[idx][idy] == 0 {
                    e_x = idx;
                    e_y = idy;
                    empty = false;
                }
            }
        }
        match empty {
            true => None,
            false=> Some((e_x, e_y))
        }
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

#[cfg(test)]
mod tests {
    use super::Board;
    use super::{new_empty, new_with_entries};

    #[test]
    fn empty_not_complete_solution() {
        let empty_board = new_empty();
        let ( _ , complete) = empty_board.is_valid_solution();
        assert_eq!( complete, false );
    }

    #[test]
    fn test_a_valid_solution() {
        let a_board = new_with_entries(
            [[ 4,5,3, 9,2,7, 1,8,6],
             [ 6,8,1, 4,3,5, 7,9,2],
             [ 7,9,2, 6,8,1, 3,4,5],

             [ 5,1,4, 7,9,3, 6,2,8],
             [ 3,2,6, 5,1,8, 4,7,9],
             [ 8,7,9, 2,6,4, 5,1,3],

             [ 9,3,7, 8,4,6, 2,5,1],
             [ 1,4,8, 3,5,2, 9,6,7],
             [ 2,6,5, 1,7,9, 8,3,4]]);
        let (valid, _ ) = a_board.is_valid_solution();
        assert_eq!( valid, true );
    }

    #[test]
    fn test_a_valid_incomplete_solution() {
        let a_board = new_with_entries(
            [[ 4,5,3, 9,2,7, 1,8,6],
             [ 6,8,0, 4,3,5, 7,9,2],
             [ 7,9,2, 6,8,0, 3,4,5],

             [ 5,1,4, 7,9,3, 6,2,8],
             [ 3,2,6, 5,1,0, 4,0,9],
             [ 8,0,9, 2,6,4, 5,1,3],

             [ 9,3,7, 8,4,6, 2,5,1],
             [ 1,4,8, 3,5,2, 9,0,7],
             [ 0,6,5, 1,7,9, 8,3,4]]);
        let (valid, complete) = a_board.is_valid_solution();
        assert_eq!((valid, complete), (true, false));
    }

    #[test]
    fn test_valid_coordinates() {
        let a_board = new_with_entries(
            [[ 4,5,3, 9,2,7, 1,8,6],
             [ 6,8,0, 4,3,5, 7,9,2],
             [ 7,9,2, 6,8,0, 3,4,5],

             [ 5,1,4, 7,9,3, 6,2,8],
             [ 3,2,6, 5,0,0, 4,0,9],
             [ 8,0,9, 2,6,4, 5,1,3],

             [ 9,3,7, 8,4,6, 2,5,1],
             [ 1,4,8, 3,5,2, 9,0,7],
             [ 0,6,5, 1,7,9, 8,3,4]]);
        let valid_pos_1 = a_board.get_valid_pos(1,2);
        assert_eq!( valid_pos_1, vec!(1) );
        let valid_pos_2 = a_board.get_valid_pos(4,5);
        assert_eq!( valid_pos_2, vec!(1,8) );
    }
}
