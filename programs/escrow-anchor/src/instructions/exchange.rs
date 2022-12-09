use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::{Exchange};

pub fn handler(ctx: Context<Exchange>) -> Result<()> {
    let bump = ctx.accounts.escrow_account.vault_authority_bump;
    let seeds = vec![bump];
    let seeds = vec![ctx.accounts.initializer.key.as_ref(), b"vault-authority".as_ref(), ctx.accounts.escrow_account.offer_id.as_bytes().as_ref(), seeds.as_slice()];
    let seeds = vec![seeds.as_slice()];
    let seeds = seeds.as_slice();
    token::transfer(
        ctx.accounts.into_transfer_to_initializer_context(),
        ctx.accounts.escrow_account.amount_of_receive_token,
    )?;

    token::transfer(
        ctx.accounts
            .into_transfer_to_taker_context()
            .with_signer(seeds),
        ctx.accounts.escrow_account.amount_of_release_token,
    )?;

    token::close_account(
        ctx.accounts
            .into_close_context()
            .with_signer(seeds),
    )?;

    Ok(())
}