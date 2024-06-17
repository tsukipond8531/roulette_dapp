use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;

use crate::errors::BetError;
use crate::state::Bet;

pub fn get_random_value(randomness_account_data: &AccountInfo, clock: &Clock) -> Result<[u8; 32]> {
    let randomness_data = RandomnessAccountData::parse(randomness_account_data.data.borrow())?;
    randomness_data
        .get_value(clock)
        .map_err(|_| BetError::RandomnessNotResolved.into())
}

pub fn determine_bet_outcome(bet: &Bet, revealed_random_value: [u8; 32]) -> bool {
    let random_number = revealed_random_value[0] % 37; // Roulette numbers 0-36
    let random_is_black = match random_number {
        2 | 4 | 6 | 8 | 10 | 11 | 13 | 15 | 17 | 20 | 22 | 24 | 26 | 28 | 29 | 31 | 33 | 35 => true,
        1 | 3 | 5 | 7 | 9 | 12 | 14 | 16 | 18 | 19 | 21 | 23 | 25 | 27 | 30 | 32 | 34 | 36 => false,
        0 => false, // 0 is green
        _ => false, // Default case for invalid numbers
    };

    let mut bet_won = bet.bet_number == random_number;

    // If the bet number does not match and is not 0, check the color
    if !bet_won && bet.bet_number != 0 {
        bet_won = bet.is_black == random_is_black;
    }

    bet_won
}

pub fn transfer_funds_to_authority(bet: &Account<Bet>, bet_authority: &Signer) {
    let lamports = **bet.to_account_info().lamports.borrow();
    **bet.to_account_info().lamports.borrow_mut() = 0;
    **bet_authority.to_account_info().lamports.borrow_mut() += lamports;
}
