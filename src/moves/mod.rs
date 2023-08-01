use crate::bitboard::{BitBoard, BoardRepresentation};

mod pawn;

pub trait MoveGenerator {
    fn generate_moves(
        &self,
        initial_position: &BitBoard,
        board: &BoardRepresentation,
    ) -> Vec<BitBoard>;
}
