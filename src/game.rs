use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TileState {
    Empty,
    X,
    O,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WinCondition {
    NoWin,
    XWin,
    OWin,
    Stalemate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub tiles: [[TileState; 3]; 3],
    pub win_condition: WinCondition,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            tiles: [[TileState::Empty; 3]; 3],
            win_condition: WinCondition::NoWin,
        }
    }

    pub fn make_move(&mut self, x: usize, y: usize, state: TileState) -> Result<(), &'static str> {
        if x >= 3 || y >= 3 {
            return Err("Invalid coordinates");
        }

        if self.tiles[x][y] != TileState::Empty {
            return Err("Tile already occupied");
        }

        if self.win_condition != WinCondition::NoWin {
            return Err("Game already finished");
        }

        self.tiles[x][y] = state;
        self.update_win_condition();
        Ok(())
    }

    pub fn reset(&mut self) {
        self.tiles = [[TileState::Empty; 3]; 3];
        self.win_condition = WinCondition::NoWin;
    }

    fn update_win_condition(&mut self) {
        // Check rows
        for row in 0..3 {
            if self.tiles[row][0] != TileState::Empty
                && self.tiles[row][0] == self.tiles[row][1]
                && self.tiles[row][1] == self.tiles[row][2]
            {
                self.win_condition = match self.tiles[row][0] {
                    TileState::X => WinCondition::XWin,
                    TileState::O => WinCondition::OWin,
                    _ => unreachable!(),
                };
                return;
            }
        }

        // Check columns
        for col in 0..3 {
            if self.tiles[0][col] != TileState::Empty
                && self.tiles[0][col] == self.tiles[1][col]
                && self.tiles[1][col] == self.tiles[2][col]
            {
                self.win_condition = match self.tiles[0][col] {
                    TileState::X => WinCondition::XWin,
                    TileState::O => WinCondition::OWin,
                    _ => unreachable!(),
                };
                return;
            }
        }

        // Check diagonals
        if self.tiles[0][0] != TileState::Empty
            && self.tiles[0][0] == self.tiles[1][1]
            && self.tiles[1][1] == self.tiles[2][2]
        {
            self.win_condition = match self.tiles[0][0] {
                TileState::X => WinCondition::XWin,
                TileState::O => WinCondition::OWin,
                _ => unreachable!(),
            };
            return;
        }

        if self.tiles[0][2] != TileState::Empty
            && self.tiles[0][2] == self.tiles[1][1]
            && self.tiles[1][1] == self.tiles[2][0]
        {
            self.win_condition = match self.tiles[0][2] {
                TileState::X => WinCondition::XWin,
                TileState::O => WinCondition::OWin,
                _ => unreachable!(),
            };
            return;
        }

        // Check for stalemate
        let mut all_filled = true;
        for row in 0..3 {
            for col in 0..3 {
                if self.tiles[row][col] == TileState::Empty {
                    all_filled = false;
                    break;
                }
            }
            if !all_filled {
                break;
            }
        }

        if all_filled {
            self.win_condition = WinCondition::Stalemate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = GameState::new();
        assert_eq!(game.win_condition, WinCondition::NoWin);
        for row in 0..3 {
            for col in 0..3 {
                assert_eq!(game.tiles[row][col], TileState::Empty);
            }
        }
    }

    #[test]
    fn test_make_move() {
        let mut game = GameState::new();
        assert!(game.make_move(0, 0, TileState::X).is_ok());
        assert_eq!(game.tiles[0][0], TileState::X);
        assert!(game.make_move(0, 0, TileState::O).is_err());
    }

    #[test]
    fn test_win_condition() {
        let mut game = GameState::new();
        game.make_move(0, 0, TileState::X).unwrap();
        game.make_move(0, 1, TileState::X).unwrap();
        game.make_move(0, 2, TileState::X).unwrap();
        assert_eq!(game.win_condition, WinCondition::XWin);
    }
}
