use std::fmt::Display;

/// Represent squares on the chess board
/// It can be the white pieces, the kings, the squares attacked by a piece, etc.
/// Each subsequent byte represent a row on the board, starting with row 1 (bottom to top)
pub struct BitBoard {
    board: u64,
}

pub struct BoardRepresentation {
    white: BitBoard,
    black: BitBoard,
    kings: BitBoard,
    queens: BitBoard,
    rooks: BitBoard,
    bishops: BitBoard,
    knights: BitBoard,
    pawns: BitBoard,
}

impl Default for BoardRepresentation {
    /// The default board representation is the starting position
    fn default() -> Self {
        BoardRepresentation {
            white: BitBoard {
                board: 0xFF_FF_00_00_00_00_00_00,
            },
            black: BitBoard {
                board: 0x00_00_00_00_00_00_FF_FF,
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
        for position in 0..64 {
            let board_position = 1 << position;
            if self.white.board & board_position != 0 || self.black.board & board_position != 0 {
                write!(f, "{}", self.get_square_display(board_position))?;
            } else {
                write!(f, ".")?;
            }
            if position % 8 == 7 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl BoardRepresentation {
    pub fn get_square_display(&self, board_position: u64) -> String {
        let mut type_string = if self.kings.board & board_position != 0 {
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
            type_string = type_string.to_uppercase();
        }
        type_string
    }
}
