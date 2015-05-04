mod board;

fn main() {
    let game_board = board::new_empty();
    game_board.display();
}

#[cfg(test)]
mod test {
    #[test]
    fn print_empty_board() {
        let a_board = board::new_empty();
        a_board.display();
    }
}

