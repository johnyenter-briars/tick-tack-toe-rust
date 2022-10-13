use rand::seq::SliceRandom;
use std::io;
const VALID_CONFS: [(usize, usize, usize); 6]  = [
    (0, 1, 2),
    (0, 3, 6),
    (0, 4, 8),
    (2, 5, 8),
    (6, 7, 8),
    (2, 4, 6),
];

#[derive(Debug)]
struct BoardState {
    positions: Vec<char>,
    game_going: bool,
}

impl BoardState {
    fn new() -> BoardState {
        BoardState {
            positions: vec!['-', '-', '-', '-', '-', '-', '-', '-', '-'],
            game_going: true,
        }
    }

    fn get_format(&self) -> String {
        let mut s = "".to_string();
        for i in 0..9 {
            if i % 3 == 0 && i != 0 {
                s += "\n";
            }
            let mut formatted_string = self.positions[i].to_string().clone();
            formatted_string.push(' ');
            s += &formatted_string;
        }
        s
    }

    fn player_moves(&mut self, index: usize) {
        self.positions[index] = 'X';
    }

    fn ai_moves(&mut self) {
        let mut valid_positions: Vec<usize> = Vec::new();

        for (index, token) in self.positions.iter().enumerate() {
            if token == &'-' {
                valid_positions.push(index);
            }
        }

        let choice_vec: Vec<&usize> = valid_positions
            .choose_multiple(&mut rand::thread_rng(), 1)
            .collect();

        let choice: usize = **choice_vec.first().unwrap();

        self.positions[choice] = 'O';
    }

    fn game_going(&self) -> bool {
        self.game_going
    }

    fn check_for_winner(&mut self) -> Option<String> {
        for player in ['X', 'O'] {
            for conf in VALID_CONFS {
                if self.positions[conf.0] == player
                    && self.positions[conf.1] == player
                    && self.positions[conf.2] == player
                {
                    self.game_going = false;

                    return Some(player.to_string());
                }
            }
        }
        None
    }
}

fn main() {
    let mut main_board = BoardState::new();

    println!("{}", main_board.get_format());
    while main_board.game_going() {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("\n---------\n");
                main_board.player_moves(input.replace('\n', "").parse::<usize>().unwrap());
                main_board.ai_moves();
                println!("{}", main_board.get_format());
                println!("\n---------\n");
                match main_board.check_for_winner() {
                    Some(winner) => {
                        println!("Winner is {}!", winner);
                        break;
                    }
                    None => continue,
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
