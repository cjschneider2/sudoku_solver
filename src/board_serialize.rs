use board::Board;
use board::{new_empty};

#[allow(dead_code)]
pub fn serialize() -> () {
    ()
}

pub fn deserialize(input_str:&str) -> Board {
    // Do some quick error checking
    // we need 81 entries for a full board
    assert!(input_str.len() == 81);

    // create a new empty board
    let mut board = new_empty();

    // we'll then parse/fill in all the entries for the board using
    // an iterator from the string.
    let mut x = 0;
    let mut y = 0;
    for chr in input_str.chars() {
        // De-char-ify the input... there should really be a fn for this in std
        let i = match chr {
            '0'...'9' => (chr as usize)-48,
            _ => 0,
        };
        match i {
            1...10 => board.insert(i, x, y),
            0 => board.insert(0, x, y),
            _ => print!("{}",i),
        }
        y += 1;
        if y > 8 { y = y % 9; x += 1; }
    }
    // and return what we've got
    board
}

#[cfg(test)]
mod test {
    use std::io::prelude::*;
    use std::path::Path;
    use std::fs;
    use std::fs::File;
    use std::error::Error;

    use board::Board;
    use board::{new_empty, new_with_entries};

    use solver::solve_with_backtracing;

    #[test]
    fn test_deserialize_w_zero_as_placeholders() {
        let test_str = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
        let test_board = new_with_entries(
        [[ 0,0,3, 0,2,0, 6,0,0],
         [ 9,0,0, 3,0,5, 0,0,1],
         [ 0,0,1, 8,0,6, 4,0,0],

         [ 0,0,8, 1,0,2, 9,0,0],
         [ 7,0,0, 0,0,0, 0,0,8],
         [ 0,0,6, 7,0,8, 2,0,0],

         [ 0,0,2, 6,0,9, 5,0,0],
         [ 8,0,0, 2,0,3, 0,0,9],
         [ 0,0,5, 0,1,0, 3,0,0]]);
        let mut input_board = super::deserialize(&test_str);
        assert_eq!(input_board, test_board);
    }
    #[test]
    fn test_deserialize_w_dots_as_placeholders() {
        let test_str = "..3.2.6..9..3.5..1..18.64....81.29..7.......8..67.82....26.95..8..2.3..9..5.1.3..";
        let test_board = new_with_entries(
        [[ 0,0,3, 0,2,0, 6,0,0],
         [ 9,0,0, 3,0,5, 0,0,1],
         [ 0,0,1, 8,0,6, 4,0,0],

         [ 0,0,8, 1,0,2, 9,0,0],
         [ 7,0,0, 0,0,0, 0,0,8],
         [ 0,0,6, 7,0,8, 2,0,0],

         [ 0,0,2, 6,0,9, 5,0,0],
         [ 8,0,0, 2,0,3, 0,0,9],
         [ 0,0,5, 0,1,0, 3,0,0]]);
        let mut input_board = super::deserialize(&test_str);
        assert_eq!(input_board, test_board);
    }
    #[test]
    fn test_sudoku_test_file() {
        let mut all_solved = false;
        // we'll enumerate our sample sudoku tests from the 'tests.txt' file,
        // one sudoku puzzle per line.
        let path = Path::new("./tests/tests.txt"); // starts in the main dir
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(E) => panic!("couldn't open file {}: {}", path.display(),
                            Error::description(&E)),
        };
        let mut string = String::new();
        file.read_to_string(&mut string);
        let entries: Vec<&str> = string.split_terminator("\r\n").collect();

        let mut solved_cnt = 0;
        let mut cnt = 0;
        for ent in entries{
            let mut input_board = Box::new(super::deserialize(&ent));
            let solved = solve_with_backtracing(&mut input_board);
            println!("{}", solved);
            if solved { solved_cnt += 1; }
            cnt += 1;
        }
        if cnt == solved_cnt { all_solved = true; }
        assert!(all_solved);
    }
}
