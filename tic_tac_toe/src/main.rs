use std::io;
use std::char;

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

    fn place_token(&mut self, position: u32) -> bool {
        self.board += 2u16.pow(position - 1);
        self.check_win()
    }
}

fn main() {
    println!("Welcome to tic tac toe");
    let mut turn = 0;
    let mut player1 = Player {
        board: 0,
        token: 'X',
    };
    let mut player2 = Player {
        board: 0,
        token: 'O',
    };
    while turn <= 9 {
        draw_board(&player1, &player2);
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
    let mut output = String::with_capacity(31);
    for n in 0..=8 {
        if n != 0 && n % 3 == 0 {
            output += "\n- - - \n";
        }
        let token = if check_bit(player1.board, n) {
            player1.token
        } else if check_bit(player2.board, n) {
            player2.token
        } else {
            char::from_digit((n + 1) as u32, 10).unwrap_or(' ')
        };
        output.push(token);
        if (n + 1) % 3 != 0 {
            output.push('|');
        }
    }
    println!("{}", output);
}

fn get_move(board1: u16, board2: u16) -> u32 {
    loop {
        println!("Please input a position from 1-9.");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        return input;
    }
}

fn check_bit(input: u16, n: u8) -> bool {
    input & (1 << n) != 0
}
