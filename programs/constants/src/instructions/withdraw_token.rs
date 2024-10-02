use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::errors::PresaleError;
use crate::state::PresaleInfo;

#[derive(Accounts)]
#[instruction(
    bump: u8
)]
pub struct WithdrawToken<'info> {
    // Presale token accounts
    #[account(mut)]
    pub mint_account: Box<Account<'info, token::Mint>>,

    #[account(
        mut,
        associated_token::mint = presale_token_mint_account,
        associated_token::authority = admin,
    )]
    pub admin_associated_token_account: Account<'info, token::TokenAccount>,

    #[account(
        mut,
        associated_token::mint = presale_token_mint_account,
        associated_token::authority = presale_info,
    )]
    pub presale_associated_token_account: Box<Account<'info, token::TokenAccount>>,

    #[account(mut)]
    pub presale_token_mint_account: Account<'info, token::Mint>,

    #[account(
        mut,
        seeds = [
            b"presale",
            presale_info.authority.as_ref()
        ],
        bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    #[account(
        mut,
        constraint = admin.key() == presale_info.authority
    )]
    pub admin: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

pub fn withdraw_token(ctx: Context<WithdrawToken>, amount: u64, bump: u8) -> Result<()> {
    let presale_info = &mut ctx.accounts.presale_info;

    if presale_info.deposit_token_amount < amount {
        return Err(PresaleError::InsufficientFund.into());
    }

    presale_info.deposit_token_amount = presale_info.deposit_token_amount - amount;

    msg!(
        "Transferring presale tokens to buyer {}...",
        &ctx.accounts.admin.key()
    );
    msg!(
        "Mint: {}",
        &ctx.accounts.mint_account.to_account_info().key()
    );
    msg!(
        "From Token Address: {}",
        &ctx.accounts.presale_associated_token_account.key()
    );
    msg!(
        "To Token Address: {}",
        &ctx.accounts.admin_associated_token_account.key()
    );
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx
                    .accounts
                    .presale_associated_token_account
                    .to_account_info(),
                to: ctx
                    .accounts
                    .admin_associated_token_account
                    .to_account_info(),
                authority: ctx.accounts.presale_info.to_account_info(),
            },
            &[&[
                b"presale" as &[u8],
                ctx.accounts.presale_info.authority.as_ref(),
                &[bump],
            ][..]],
        ),
        amount,
    )?;

    msg!("Withdrew presale tokens successfully.");

    Ok(())
}
