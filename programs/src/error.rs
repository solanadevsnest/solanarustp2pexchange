use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("The stage specified for exchange or cancel is invalid.")]
    InvalidStage,
    #[msg("The available funds are insufficient for this operation.")]
    InsufficientFunds,
    #[msg("The specified mint account for trade is invalid.")]
    InvalidMint,
    #[msg("A necessary mint for the trade is missing.")]
    MissingMint,
    #[msg("The trade type is invalid, potentially due to missing mint addresses.")]
    InvalidTradeType,
    #[msg("Invalid mint association between the provided token accounts.")]
    InvalidAccount,
    #[msg("Duplicate mint accounts are not allowed for this operation.")]
    DuplicateMint,
    #[msg("The account does not have a valid owner for this transaction.")]
    InvalidOwner,
    #[msg("The specified partner is not valid for this trade.")]
    InvalidPartner,
    #[msg("Both trade value and receive value must be greater than zero.")]
    ZeroValue,
    #[msg("The instruction data is missing necessary parameters.")]
    MissingParams,
}
