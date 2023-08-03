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
        if move_one & (board.white | board.black) == 0 && initial_position.board < 1 << 56 {
            moves.push(move_one);
        }
        if *initial_position & 0x00_00_00_00_00_00_FF_00 != 0 {
            let move_two = *initial_position << 16;
            if move_two & (board.white | board.black) == 0
                && move_one & (board.white | board.black) == 0
            {
                moves.push(move_two);
            }
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
        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0].board, 1 << 16);
        assert_eq!(moves[1].board, 1 << 24);
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

        let mut board = BoardRepresentation::default();
        board.white |= BitBoard::new(1 << 24);
        let moves = generator.generate_moves(&BitBoard::new(1 << 8), &board);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].board, 1 << 16);

        let mut board = BoardRepresentation::default();
        board.black |= BitBoard::new(1 << 24);
        let moves = generator.generate_moves(&BitBoard::new(1 << 8), &board);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].board, 1 << 16);
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
