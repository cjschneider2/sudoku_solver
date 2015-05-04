pub struct Board {
    entries: [[usize; 9]; 9]
}

impl Board {
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
