use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("must be an authorized caller")]
    Unauthorized,
    #[msg("invalid asset given")]
    InvalidAsset,
    #[msg("too many pending deposits")]
    TooManyDeposits,
    #[msg("insufficient funds")]
    Insufficientfunds,
}