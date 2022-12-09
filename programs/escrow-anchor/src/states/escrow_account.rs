use anchor_lang::prelude::*;

pub const ESCROW_ACCOUNT_LEN: usize = 8     // ANCHOR DISCRIMINATOR
    + 32                                    // INITIALIZER WALLET KEY
    + 1                                     // bump for vault account
    + 1                                     // bump for vault authority
    + 1                                     // bump for vault escrow account
    + 32                                    // offer_id
    + 32                                    // INITIALIZER RELEASE TOKEN ACCOUNT KEY
    + 32                                    // INITIALIZER RECEIVE TOKEN ACCOUNT KEY
    + 8                                     // Amount of Release Token
    + 8;                                    // Amount of Receive Token

#[account]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub vault_account_bump: u8,
    pub vault_authority_bump: u8,
    pub escrow_account_bump: u8,
    pub offer_id: String,
    pub initializer_release_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub amount_of_release_token: u64,
    pub amount_of_receive_token: u64
}