use crate::bitboard::BoardRepresentation;

mod bitboard;
mod move_parser;
mod moves;

fn main() {
    let mut board = BoardRepresentation::default();
    loop {
        println!("{board}");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        let parsed_move = move_parser::parse_move(input);
        match parsed_move {
            Ok(m) => {
                println!("Move:\n{:}", m.to);
                board.make_move(m);
            }
            Err(e) => println!("Error: {e}"),
        }
    }
}
