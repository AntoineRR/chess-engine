use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, BitOrAssign, Shl},
};

#[derive(Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
pub struct Move {
    pub piece: PieceType,
    pub to: BitBoard,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub fn new(piece: PieceType, to: BitBoard, promotion: Option<PieceType>) -> Self {
        Move {
            piece,
            to,
            promotion,
        }
    }
}

/// Represent squares on the chess board
/// It can be the white pieces, the kings, the squares attacked by a piece, etc.
/// Each subsequent byte represent a row on the board, starting with row 1 (bottom to top)
/// https://www.chessprogramming.org/Square_Mapping_Considerations#LittleEndianRankFileMapping
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard {
    pub board: u64,
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![[""; 8]; 8];
        for position in 0..64 {
            let board_position = 1 << position;
            lines[position / 8][position % 8] = if self.board & board_position != 0 {
                "x"
            } else {
                "."
            };
        }
        for line in lines.iter().rev() {
            writeln!(f, "{}", line.join(""))?;
        }
        Ok(())
    }
}

impl BitBoard {
    pub fn new(board: u64) -> Self {
        BitBoard { board }
    }
}

impl Shl<u8> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        BitBoard {
            board: self.board << rhs,
        }
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard {
            board: self.board & rhs.board,
        }
    }
}

impl BitAnd<u64> for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: u64) -> Self::Output {
        BitBoard {
            board: self.board & rhs,
        }
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard {
            board: self.board | rhs.board,
        }
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.board |= rhs.board;
    }
}

impl PartialEq<u64> for BitBoard {
    fn eq(&self, other: &u64) -> bool {
        self.board == *other
    }
}

pub struct BoardRepresentation {
    pub white: BitBoard,
    pub black: BitBoard,
    pub kings: BitBoard,
    pub queens: BitBoard,
    pub rooks: BitBoard,
    pub bishops: BitBoard,
    pub knights: BitBoard,
    pub pawns: BitBoard,
}

impl Default for BoardRepresentation {
    /// The default board representation is the starting position
    fn default() -> Self {
        BoardRepresentation {
            white: BitBoard {
                board: 0x00_00_00_00_00_00_FF_FF,
            },
            black: BitBoard {
                board: 0xFF_FF_00_00_00_00_00_00,
            },
            kings: BitBoard {
                board: 0x08_00_00_00_00_00_00_08,
            },
            queens: BitBoard {
                board: 0x10_00_00_00_00_00_00_10,
            },
            rooks: BitBoard {
                board: 0x81_00_00_00_00_00_00_81,
            },
            bishops: BitBoard {
                board: 0x24_00_00_00_00_00_00_24,
            },
            knights: BitBoard {
                board: 0x42_00_00_00_00_00_00_42,
            },
            pawns: BitBoard {
                board: 0x00_FF_00_00_00_00_FF_00,
            },
        }
    }
}

impl Display for BoardRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![vec!["".to_string(); 8]; 8];
        for position in 0..64 {
            let board_position = 1 << position;
            let mut symbol = if self.kings.board & board_position != 0 {
                "k".to_string()
            } else if self.queens.board & board_position != 0 {
                "q".to_string()
            } else if self.rooks.board & board_position != 0 {
                "r".to_string()
            } else if self.bishops.board & board_position != 0 {
                "b".to_string()
            } else if self.knights.board & board_position != 0 {
                "n".to_string()
            } else if self.pawns.board & board_position != 0 {
                "p".to_string()
            } else {
                ".".to_string()
            };
            if self.white.board & board_position != 0 {
                symbol.make_ascii_uppercase();
            }
            lines[position / 8][position % 8] = symbol;
        }
        for line in lines.iter().rev() {
            writeln!(f, "{}", line.join(""))?;
        }
        Ok(())
    }
}

impl BoardRepresentation {
    pub fn make_move(&mut self, _m: Move) {}
}
