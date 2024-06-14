pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;

declare_id!("9UiNkKquaH2AhXmQC7QKfrtyZoF3mHJzJcmYxvDdir3H");

#[program]
pub mod roulette_dapp {
    use super::*;

    pub fn initialize(ctx: Context<InitializeBet>, bet_number: u8, is_black: bool) -> Result<()> {
        _initialize_bet(ctx, bet_number, is_black)
    }
}
