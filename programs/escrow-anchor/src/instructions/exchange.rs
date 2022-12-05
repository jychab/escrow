use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::{Exchange};

pub fn handler(ctx: Context<Exchange>) -> Result<()> {
    let offer_key = ctx.accounts.escrow_account.offer.key();
    let vault_authority_seed = [offer_key.as_ref(), b"vault-authority".as_ref()];
    let (_vault_authority, vault_authority_bump) =
        Pubkey::find_program_address(&vault_authority_seed, ctx.program_id);
    let authority_seeds = [&vault_authority_seed[..], &[&[vault_authority_bump]]];

    token::transfer(
        ctx.accounts.into_transfer_to_initializer_context(),
        ctx.accounts.escrow_account.amount_of_receive_token,
    )?;

    token::transfer(
        ctx.accounts
            .into_transfer_to_taker_context()
            .with_signer(&authority_seeds[..]),
        ctx.accounts.escrow_account.amount_of_release_token,
    )?;

    token::close_account(
        ctx.accounts
            .into_close_context()
            .with_signer(&authority_seeds[..]),
    )?;

    Ok(())
}