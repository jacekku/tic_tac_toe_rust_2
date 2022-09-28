mod tests;

use std::num::ParseIntError;

#[derive(PartialEq)]
enum Field {
    X,
    O,
    E,
}
impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::E => write!(f, " "),
            Field::X => write!(f, "X"),
            Field::O => write!(f, "O"),
        }
    }
}

struct Board {
    fields: Vec<Field>,
}

impl Board {
    fn can_place(&mut self, index: usize) -> bool {
        if index >= self.fields.len() {
            return false;
        }
        if self.fields[index] == Field::E {
            return true;
        }
        return false;
    }

    fn place(&mut self, index: usize, player: &Field) {
        match player {
            Field::X => self.fields[index] = Field::X,
            Field::O => self.fields[index] = Field::O,
            Field::E => self.fields[index] = Field::E,
        }
    }

    fn check_win(&self) -> bool {
        let mut outcome = false;
        let combinations = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        for blocks in combinations {
            let first = &self.fields[blocks[0]];
            if first == &Field::E {
                continue;
            }
            let all_the_same = blocks.iter().all(|idx| &self.fields[*idx] == first);
            if all_the_same {
                outcome = true;
            }
        }

        return outcome;
    }

    fn check_draw(&self) -> bool {
        for field in &self.fields {
            if field == &Field::E {
                return false;
            }
        }
        return true;
    }
}

struct GameState {
    player: Field,
}
impl GameState {
    fn flip_player(&mut self) {
        match self.player {
            Field::E => self.player = Field::X,
            Field::X => self.player = Field::O,
            Field::O => self.player = Field::X,
        }
    }
}

fn pretty_print_board(board: &Board) {
    println!("======");
    println!(
        "{}|{}|{}",
        board.fields[0], board.fields[1], board.fields[2]
    );
    println!("------");
    println!(
        "{}|{}|{}",
        board.fields[3], board.fields[4], board.fields[5]
    );
    println!("------");
    println!(
        "{}|{}|{}",
        board.fields[6], board.fields[7], board.fields[8]
    );
    println!("======");
}

fn main() {
    let mut game_state = GameState { player: Field::X };
    let mut board = Board {
        fields: Vec::from([
            Field::E,
            Field::E,
            Field::E,
            Field::E,
            Field::E,
            Field::E,
            Field::E,
            Field::E,
            Field::E,
        ]),
    };
    println!("Tic Tac Toe");

    let result: String = loop {
        println!("{} places", game_state.player);
        pretty_print_board(&board);

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        check_if_exit(&input);

        let index = get_number(input);
        match index {
            Err(err) => println!("{}", err),
            Ok(value) => {
                if value >= 9 || !board.can_place(value) {
                    println!("Please write number between 0 and 8, and not with a filled field");
                    continue;
                }
                board.place(value, &game_state.player);
                pretty_print_board(&board);

                game_state.flip_player();

                if board.check_win() {
                    game_state.flip_player();
                    break format!("{} wins!", game_state.player);
                }
                if board.check_draw() {
                    break String::from("it's a draw!");
                }
            }
        }
    };
    println!("{}", result);
}

fn get_number(input: String) -> Result<usize, ParseIntError> {
    let index: Result<usize, ParseIntError> = input.trim().parse();
    index
}

fn check_if_exit(input: &String) {
    if input.trim() == "quit" || input.trim() == "exit" {
        std::process::exit(0);
    }
}
