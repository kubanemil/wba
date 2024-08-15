use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("An account should be mutable")]
    ImmutableAccountError,
    #[msg("Account does not have token_a")]
    NoTokenAError,
}
