use std::io;
use std::io::Write;
use std::char;

struct Player {
    board: u16,
    token: char,
}

impl Player {
    fn place_token(&mut self, position: u8) -> bool {
        self.board += 2u16.pow((position - 1) as u32);
        self.check_win()
    }

    fn check_win(&self) -> bool {
        let horizontal_mask = 0b0111;
        let vertical_mask = 0b0100_1001;
        self.check_diagonal_win() || self.check_orthagonal_win(horizontal_mask, 3) || self.check_orthagonal_win(vertical_mask, 1)
    }

    fn check_orthagonal_win(&self, mut mask: u16, bit_shift: u8) -> bool {
        for _ in 0..3 {
            if mask & self.board == mask {
                return true;
            }
            mask <<= bit_shift;
        }
        false
    }

    fn check_diagonal_win(&self) -> bool {
        let left_diagonal_mask = 0b1_0001_0001;
        let right_diagonal_mask = 0b0101_0100;
        self.board & left_diagonal_mask == left_diagonal_mask || self.board & right_diagonal_mask == right_diagonal_mask 
    }
}

fn main() {
    println!("Welcome to Tic Tac Toe!");
    let mut turn = 0;
    let mut player1 = Player {
        board: 0,
        token: 'X',
    };
    let mut player2 = Player {
        board: 0,
        token: 'O',
    };
    while turn < 9 {
        draw_board(&player1, &player2);
        println!("{}'s turn.", if turn % 2 == 0 {player1.token} else {player2.token});
        let position = get_move(player1.board, player2.board);
        let game_over;
        if turn % 2 == 0 {
            game_over = player1.place_token(position);
        } else {
            game_over = player2.place_token(position);
        }
        if game_over {
            break;
        }
        turn += 1;
    }
}

fn draw_board(player1: &Player, player2: &Player) {
    let mut output = String::with_capacity(54);
    for n in 0..=8 {
        if n != 0 && n % 3 == 0 {
            output += "\n -   -   -\n";
        }
        let token = if check_bit(player1.board, n) {
            player1.token
        } else if check_bit(player2.board, n) {
            player2.token
        } else {
            char::from_digit((n + 1) as u32, 10).unwrap()
        };
        output += &format!(" {}", token);
        if (n + 1) % 3 != 0 {
            output += " |";
        }
    }
    println!("\n{}\n", output);
}

fn get_move(board1: u16, board2: u16) -> u8 {
    loop {
        print!("Please input a position from 1-9: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => {
                if num >= 1 && num <= 9 && !(check_bit(board1, num - 1) || check_bit(board2, num - 1)) {
                    return num;
                }
            },
            Err(_) => continue,
        };
    }
}

// Returns true if the specified bit is 1
fn check_bit(input: u16, n: u8) -> bool {
    input & (1 << n) != 0
}
