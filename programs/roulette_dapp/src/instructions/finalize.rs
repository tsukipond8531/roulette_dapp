use anchor_lang::prelude::*;

use crate::constants::BET_SEED;
use crate::states::Bet;
use crate::utils::*;
use crate::BetError;

pub fn _finalize_bet(ctx: Context<FinalizeBet>) -> Result<()> {
    let clock = Clock::get()?;
    let bet = &ctx.accounts.bet;

    require!(
        ctx.accounts.bet_authority.key() == bet.bet_authority.key(),
        BetError::UnauthorizedSigner,
    );

    let revealed_random_value = get_random_value(&ctx.accounts.randomness_account_data, &clock)?;

    let bet_won = determine_bet_outcome(bet, &revealed_random_value);

    if bet_won {
        msg!("Bet won!");
    } else {
        msg!("Bet lost!");
    }

    transfer_funds_to_authority(&ctx.accounts.bet, &ctx.accounts.bet_authority);

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
    /// CHECK: this is later handled
    pub randomness_account_data: AccountInfo<'info>,
}
