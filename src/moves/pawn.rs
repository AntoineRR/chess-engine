use crate::bitboard::{BitBoard, BoardRepresentation};

use super::MoveGenerator;

pub struct PawnMoveGenerator {}

impl MoveGenerator for PawnMoveGenerator {
    fn generate_moves(
        &self,
        initial_position: &BitBoard,
        board: &BoardRepresentation,
    ) -> Vec<BitBoard> {
        let mut moves = Vec::new();
        let move_one = *initial_position << 8;
        if move_one.clone() & (board.white | board.black) == 0 && initial_position.board < 1 << 56 {
            moves.push(move_one);
        }
        moves
    }
}

#[cfg(test)]
mod test {
    use crate::bitboard::{BitBoard, BoardRepresentation};

    use super::super::MoveGenerator;
    use super::PawnMoveGenerator;

    #[test]
    fn test_pawn_move() {
        let generator = PawnMoveGenerator {};
        let board = BoardRepresentation::default();
        let moves = generator.generate_moves(&BitBoard::new(1 << 8), &board);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].board, 1 << 16);
    }

    #[test]
    fn test_pawn_move_blocked() {
        let generator = PawnMoveGenerator {};
        let mut board = BoardRepresentation::default();
        board.white |= BitBoard::new(1 << 16);
        let moves = generator.generate_moves(&BitBoard::new(1 << 8), &board);
        assert_eq!(moves.len(), 0);

        let mut board = BoardRepresentation::default();
        board.black |= BitBoard::new(1 << 16);
        let moves = generator.generate_moves(&BitBoard::new(1 << 8), &board);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_pawn_move_outside_board() {
        let generator = PawnMoveGenerator {};
        let board = BoardRepresentation::default();
        let moves = generator.generate_moves(&BitBoard::new(1 << 56), &board);
        assert_eq!(moves.len(), 0);

        let moves = generator.generate_moves(&BitBoard::new(1 << 63), &board);
        assert_eq!(moves.len(), 0);
    }
}
