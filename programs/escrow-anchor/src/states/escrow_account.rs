use anchor_lang::prelude::*;

pub const ESCROW_ACCOUNT_LEN: usize = 8     // ANCHOR DISCRIMINATOR
    + 32                                    // INITIALIZER WALLET KEY
    + 32                                    // Escrow PDA seed
    + 32                                    // INITIALIZER RELEASE TOKEN ACCOUNT KEY
    + 32                                    // INITIALIZER RECEIVE TOKEN ACCOUNT KEY
    + 8                                     // Amount of Release Token
    + 8;                                    // Amount of Receive Token

#[account]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub offer: Pubkey,
    pub initializer_release_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub amount_of_release_token: u64,
    pub amount_of_receive_token: u64
}