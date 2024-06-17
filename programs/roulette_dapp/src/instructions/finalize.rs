use anchor_lang::prelude::*;

use crate::constants::BET_SEED;
use crate::states::Bet;
use crate::BetError;

pub fn _finalize_bet(ctx: Context<FinalizeBet>) -> Result<()> {
    let bet = &ctx.accounts.bet;

    require!(
        ctx.accounts.bet_authority.key() == bet.bet_authority.key(),
        BetError::UnauthorizedSigner,
    );

    const rnd_bet = 

    Ok(())
}

#[derive(Accounts)]
pub struct FinalizeBet<'info> {
    #[account(mut)]
    pub bet_authority: Signer<'info>,
    #[account(
        mut,
        close = bet_authority,
        seeds = [
            bet_authority.key().as_ref(),
            BET_SEED.as_bytes(),
            &[bet.bet_number],
            &[bet.is_black as u8],
        ],
        bump = bet.bump,
    )]
    pub bet: Account<'info, Bet>,
}
