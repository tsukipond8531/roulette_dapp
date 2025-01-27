use anchor_lang::prelude::*;

use crate::states::{Bet,BET_LEN};
use crate::errors::BetError;
use crate::constants::BET_SEED;

pub fn _initialize_bet(ctx: Context<InitializeBet>, bet_number:u8,is_black:bool) -> Result<()> {
    let bet = &mut ctx.accounts.bet;

    require!(bet_number <= 36, BetError::InvalidBetNumber);

    bet.bet_authority = ctx.accounts.bet_authority.key();
    bet.bump = ctx.bumps.bet;
    bet.bet_number = bet_number;
    bet.is_black = is_black;
    bet.is_even = bet_number % 2 == 0;
    bet.randomness_account = Pubkey::default(); //placeholder, will be set later

    Ok(())
}

#[derive(Accounts)]
#[instruction(bet_number:u8,is_black:bool)]
pub struct InitializeBet<'info> {
    #[account(
        init,
        payer  = bet_authority,
        space = 8 + BET_LEN,
        seeds = [
            bet_authority.key().as_ref(),
            BET_SEED.as_bytes(),
            &[bet_number],
            &[is_black as u8],
        ],
        bump    
    )]
    pub bet: Account<'info,Bet>,
    #[account(mut)]
    pub bet_authority: Signer<'info>,
    pub system_program: Program<'info,System>,
}

