use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor_tic_tac_toe {
    use super::*;

    // TODO why do we add player_two here and not in SetupGame struct?
    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.players = [ctx.accounts.player_one.key(), player_two];
        game.turn = 1;
        Ok(())
    }

    pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
        let game = &mut ctx.accounts.game;

        require!(
            game.current_player() == ctx.accounts.player.key(),
            TicTacToeError::NotPlayersTurn
        );

        game.play(&tile)
    }
}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    // we add 8 to MAXIMUM_SIZE, this is space for the discriminator which anchor sets automatically
    #[account(init, payer = player_one, space = Game::MAXIMUM_SIZE + 8)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_one: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct Game {
    players: [Pubkey; 2],          // 64
    turn: u8,                      // 1
    board: [[Option<Sign>; 3]; 3], // 9 * (1 + 1) = 18
    state: GameState,              // 32 + 1
}

impl Game {
    const MAXIMUM_SIZE: usize = 116; // max size, if board is empty then won't take all these bytes

    pub fn is_active(&self) -> bool {
        self.state == GameState::Active
    }

    fn current_player_index(&self) -> usize {
        ((self.turn - 1) % 2) as usize
    }

    pub fn current_player(&self) -> Pubkey {
        self.players[self.current_player_index()]
    }

    pub fn play(&mut self, tile: &Tile) -> Result<()> {
        if !self.is_active() {
            return err!(TicTacToeError::GameAlreadyOver);
        }
        match tile {
            tile 
            @ Tile {
                row: 0..=2, 
                column: 0..=2
            } => match self.board[tile.row as usize][tile.column as usize] {
                Some(_) => return err!(TicTacToeError::TileAlreadySet),
                None => {
                    self.board[tile.row as usize][tile.column as usize] =
                        Some(Sign::from_usize(self.current_player_index()).unwrap());
                }
            },
            _ => return err!(TicTacToeError::TileOutOfBounds),
        }

        self.update_state();
        if let GameState::Active = self.state {
            self.turn += 1;
        }

        Ok(())
    }

    fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
        let [first, second, third] = trio;
        self.board[first.0][first.1].is_some()
            && self.board[first.0][first.1] == self.board[second.0][second.1]
            && self.board[first.0][first.1] == self.board[third.0][third.1]
    }

    fn update_state(&mut self) {
        for i in 0..=2 {
            // row
            if self.is_winning_trio([(i, 0), (i, 1), (i, 2)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }
            // col 
            if self.is_winning_trio([(0, i), (1, i), (2, i)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }
        }
        //diagonal
        if self.is_winning_trio([(0,0), (1,1), (2,2)])
            || self.is_winning_trio([(0,2), (1,1), (2,0)]) {
                self.state = GameState::Won {
                    winner: self.current_player()
                };
                return;
        }

        // game not won yet, see if there is a tie or not
        for row in 0..=2 {
            for col in 0..=2 {
                if self.board[row][col].is_none() {
                    return;
                }
            }
        }
        self.state = GameState::Tie;
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Tile {
    row: u8,
    column: u8
}

#[error_code]
pub enum TicTacToeError {
    TileOutOfBounds,
    TileAlreadySet,
    GameAlreadyOver,
    NotPlayersTurn,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}

impl Default for GameState {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq)]
pub enum Sign {
    X,
    O,
}
