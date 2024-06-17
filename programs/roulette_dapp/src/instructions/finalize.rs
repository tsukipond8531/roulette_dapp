use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;

use crate::constants::BET_SEED;
use crate::states::Bet;
use crate::BetError;

pub fn _finalize_bet(ctx: Context<FinalizeBet>) -> Result<()> {
    let clock = Clock::get()?;
    let bet = &ctx.accounts.bet;

    require!(
        ctx.accounts.bet_authority.key() == bet.bet_authority.key(),
        BetError::UnauthorizedSigner,
    );

    let randomness_data =
        RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow()).unwrap();
    let revealed_random_value = randomness_data
        .get_value(&clock)
        .map_err(|_| BetError::RandomnessNotResolved)?;

    // Determine the outcome of the bet using the random value
    let random_number = revealed_random_value[0] % 37;

    let random_is_black = match random_number {
        2 | 4 | 6 | 8 | 10 | 11 | 13 | 15 | 17 | 20 | 22 | 24 | 26 | 28 | 29 | 31 | 33 | 35 => true,
        1 | 3 | 5 | 7 | 9 | 12 | 14 | 16 | 18 | 19 | 21 | 23 | 25 | 27 | 30 | 32 | 34 | 36 => false,
        0 => false, // 0 is green
        _ => return Err(BetError::RandomnessNotResolved.into()),
    };

    let mut bet_won = bet.bet_number == random_number;

    if !bet_won && bet.bet_number != 0 {
        bet_won = bet.is_black == random_is_black;
    }

    if bet_won {
        msg!("Bet won!");
    } else {
        msg!("Bet lost!");
    }

    let lamports = **ctx.accounts.bet.to_account_info().lamports.borrow();
    **ctx.accounts.bet.to_account_info().lamports.borrow_mut() = 0;
    **ctx
        .accounts
        .bet_authority
        .to_account_info()
        .lamports
        .borrow_mut() += lamports;

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
