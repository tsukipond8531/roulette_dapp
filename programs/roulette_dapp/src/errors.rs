use anchor_lang::prelude::*;

#[error_code]
pub enum BetError {
    #[msg("Bet Number must be between 0 and 36")]
    InvalidBetNumber,
    #[msg("The caller does not have bet authority")]
    UnauthorizedSigner,
    #[msg("Could not get random number value")]
    RandomnessNotResolved,
}
