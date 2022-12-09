use anchor_lang::{prelude::*};
use anchor_spl::token::{TokenAccount, Mint, Transfer};

use crate::states::{EscrowAccount, ESCROW_ACCOUNT_LEN};


#[derive(Accounts)]
#[instruction(offer_id:String, amount_of_release_token: u64, amount_of_receieve_token: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [initializer.key.as_ref(), b"vault-account".as_ref(), offer_id.as_bytes().as_ref()],
        bump,
        payer = initializer,
        token::mint = mint,
        token::authority = vault_authority,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [initializer.key.as_ref(), b"vault-authority".as_ref(), offer_id.as_bytes().as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we have checked account using seeds
    pub vault_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub initializer_release_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        seeds = [initializer.key.as_ref(), b"escrow-account".as_ref(), offer_id.as_bytes().as_ref()],
        bump,
        payer = initializer,
        space = ESCROW_ACCOUNT_LEN
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}


impl <'info> Initialize <'info> {
    pub fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .initializer_release_token_account
                .to_account_info()
                .clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.initializer.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}