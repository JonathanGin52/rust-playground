use std::io;

struct Player {
    board: u16,
    token: char,
}

impl Player {
    fn check_win(&self) -> bool {
        let win_masks = [
            0b0_0000_0111,
            0b0_0011_1000,
            0b1_1100_0000,
            0b0_0100_1001,
            0b0_1001_0010,
            0b1_0010_0100,
            0b1_0001_0001,
            0b0_0101_0100,
        ];
        for &mask in &win_masks {
            if mask & self.board == mask {
                return true
            }
        }
        false
    }
}

fn main() {
    let player1 = Player {
        board: 0,
        token: 'X',
    };
    println!("Welcome to tic tac toe");
}
