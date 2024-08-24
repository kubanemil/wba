use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,

    #[msg("ED25519 does not match")]
    Ed25519Program,
    Ed25519Account,
    Ed25519DataLength,
    Ed25519Pubkey,
    Ed25519Signature
}
