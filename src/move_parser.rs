use crate::bitboard::{BitBoard, Move, PieceType};

const ROOK_PREFIX: char = 'R';
const KNIGHT_PREFIX: char = 'N';
const BISHOP_PREFIX: char = 'B';
const QUEEN_PREFIX: char = 'Q';
const KING_PREFIX: char = 'K';

fn parse_square(chars: &mut std::str::Chars) -> Result<BitBoard, String> {
    let file = chars.next().ok_or("Invalid move")?;
    let rank = chars.next().ok_or("Invalid move")?;
    let file = match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return Err("Invalid move".to_string()),
    };
    let rank = match rank {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => return Err("Invalid move".to_string()),
    };
    Ok(BitBoard::new(1 << (file + rank * 8)))
}

pub fn parse_move(move_string: &str) -> Result<Move, String> {
    let mut chars = move_string.chars();
    let piece = chars.next().ok_or("Invalid move")?;
    let piece = match piece {
        ROOK_PREFIX => PieceType::Rook,
        KNIGHT_PREFIX => PieceType::Knight,
        BISHOP_PREFIX => PieceType::Bishop,
        QUEEN_PREFIX => PieceType::Queen,
        KING_PREFIX => PieceType::King,
        _ => PieceType::Pawn,
    };
    let (to, promotion) = if piece == PieceType::Pawn {
        chars.next().ok_or("Invalid move")?; // Skip the rank to get promotion
        let promotion = chars.next().map(|c| match c {
            ROOK_PREFIX => PieceType::Rook,
            KNIGHT_PREFIX => PieceType::Knight,
            BISHOP_PREFIX => PieceType::Bishop,
            QUEEN_PREFIX => PieceType::Queen,
            _ => PieceType::Pawn,
        });
        (parse_square(&mut move_string.chars())?, promotion)
    } else {
        (parse_square(&mut chars)?, None)
    };
    Ok(Move::new(piece, to, promotion))
}

#[cfg(test)]
mod test {
    use crate::{bitboard::PieceType, move_parser::parse_move};

    #[test]
    fn test_pawn_move() {
        let parsed_move = parse_move("e4");
        assert!(parsed_move.is_ok());
        let parsed_move = parsed_move.unwrap();
        assert_eq!(parsed_move.piece, PieceType::Pawn);
        assert_eq!(parsed_move.to.board, 1 << 28);
        assert!(parsed_move.promotion.is_none());
    }

    #[test]
    fn test_knight_move() {
        let parsed_move = parse_move("Nf3");
        assert!(parsed_move.is_ok());
        let parsed_move = parsed_move.unwrap();
        assert_eq!(parsed_move.piece, PieceType::Knight);
        assert_eq!(parsed_move.to.board, 1 << 21);
        assert!(parsed_move.promotion.is_none());
    }

    #[test]
    fn test_bishop_move() {
        let parsed_move = parse_move("Bb5");
        assert!(parsed_move.is_ok());
        let parsed_move = parsed_move.unwrap();
        assert_eq!(parsed_move.piece, PieceType::Bishop);
        assert_eq!(parsed_move.to.board, 1 << 33);
        assert!(parsed_move.promotion.is_none());
    }

    #[test]
    fn test_rook_move() {
        let parsed_move = parse_move("Rd1");
        assert!(parsed_move.is_ok());
        let parsed_move = parsed_move.unwrap();
        assert_eq!(parsed_move.piece, PieceType::Rook);
        assert_eq!(parsed_move.to.board, 1 << 3);
        assert!(parsed_move.promotion.is_none());
    }

    #[test]
    fn test_queen_move() {
        let parsed_move = parse_move("Qe2");
        assert!(parsed_move.is_ok());
        let parsed_move = parsed_move.unwrap();
        assert_eq!(parsed_move.piece, PieceType::Queen);
        assert_eq!(parsed_move.to.board, 1 << 12);
        assert!(parsed_move.promotion.is_none());
    }

    #[test]
    fn test_king_move() {
        let parsed_move = parse_move("Ke1");
        assert!(parsed_move.is_ok());
        let parsed_move = parsed_move.unwrap();
        assert_eq!(parsed_move.piece, PieceType::King);
        assert_eq!(parsed_move.to.board, 1 << 4);
        assert!(parsed_move.promotion.is_none());
    }
}
