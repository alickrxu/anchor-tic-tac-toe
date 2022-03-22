use anchor_lang::prelude::*;
use instructions::*;
use state::game::Tile;

pub mod errors;
pub mod state;
pub mod instructions;

declare_id!("HmAcuqLYqHf1d2KSXxiYj4bMNDdNm64hbg5hqEKCYi8Y");

#[program]
pub mod anchor_tic_tac_toe {
    use super::*;

    // TODO why do we add player_two here and not in SetupGame struct?
    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
        instructions::setup_game::setup_game(ctx, player_two)
    }

    pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
        instructions::play::play(ctx, tile)
    }
}
