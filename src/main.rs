mod board;

fn main() {
    let game_board = board::new_empty();
    game_board.display();
}

#[cfg(test)]
mod test {
    #[test]
    fn display_empty_board() {
        let a_board = super::board::new_empty();
        a_board.display();
    }
    #[test]
    fn empty_not_valid_solution() {
        let empty_board = super::board::new_empty();
        let value = empty_board.is_valid_solution();
        assert_eq!(value, false);
    }
    #[test]
    fn test_a_valid_solution() {
        let a_board = super::board::new_with_entries(
            [[ 4,5,3, 9,2,7, 1,8,6],
             [ 6,8,1, 4,3,5, 7,9,2],
             [ 7,9,2, 6,8,1, 3,4,5],

             [ 5,1,4, 7,9,3, 6,2,8],
             [ 3,2,6, 5,1,8, 4,7,9],
             [ 8,7,9, 2,6,4, 5,1,3],

             [ 9,3,7, 8,4,6, 2,5,1],
             [ 1,4,8, 3,5,2, 9,6,7],
             [ 2,6,5, 1,7,9, 8,3,4]]);
        let value = a_board.is_valid_solution();
        assert_eq!(value, true);
    }
}
