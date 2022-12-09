use anchor_lang::prelude::*;

pub mod contexts;
pub mod states;
pub mod instructions;
pub mod error;

pub use contexts::*;
declare_id!("2PPVwtG213wMbDTSTTw2E2UPJ8DTBjdxo5DkEhqJY2yB");

#[program]
pub mod escrow_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, offer_id: String, amount_of_release_token: u64, amount_of_receive_token: u64) -> Result<()> {
        instructions::initialize::handler(ctx, offer_id, amount_of_release_token, amount_of_receive_token)
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        instructions::exchange::handler(ctx)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        instructions::cancel::handler(ctx)
    }
}
