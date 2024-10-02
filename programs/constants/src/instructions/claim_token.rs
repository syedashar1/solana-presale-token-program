use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::errors::PresaleError;
use crate::state::{PresaleInfo, UserInfo};

#[derive(Accounts)]
pub struct ClaimToken<'info> {
    // Presale token accounts
    #[account(
        mut,
        constraint = token_mint.key() == presale_info.token_mint_address
    )]
    pub token_mint: Box<Account<'info, token::Mint>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = token_mint,
        associated_token::authority = buyer,
    )]
    pub token_account: Box<Account<'info, token::TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = presale_info,
    )]
    pub presale_associated_token_account: Box<Account<'info, token::TokenAccount>>,

    #[account(
        mut,
        seeds = [
            b"user",
            buyer.key().as_ref()
        ],        
        bump
    )]
    pub user_info: Box<Account<'info, UserInfo>>,

    #[account(
        mut,
        seeds = [
            b"presale",
            presale_info.authority.as_ref()
        ],
        bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

pub fn claim_token(ctx: Context<ClaimToken>, bump: u8) -> Result<()> {
    let presale_info = &mut ctx.accounts.presale_info;

    let cur_timestamp = u64::try_from(Clock::get()?.unix_timestamp).unwrap();

    // get time and compare with start and end time
    if presale_info.end_time > cur_timestamp * 1000 {
        msg!("current time: {}", cur_timestamp);
        msg!("presale end time: {}", presale_info.end_time);
        msg!("Presale not ended yet.");
        return Err(PresaleError::PresaleNotEnded.into());
    }

    let user_info = &mut ctx.accounts.user_info;
    let claim_amount = user_info.buy_token_amount;

    msg!(
        "Transferring presale tokens to buyer {}...",
        &ctx.accounts.buyer.key()
    );
    msg!("Mint: {}", &ctx.accounts.token_mint.to_account_info().key());
    msg!(
        "From Token Address: {}",
        &ctx.accounts.presale_associated_token_account.key()
    );
    msg!("To Token Address: {}", &ctx.accounts.token_account.key());
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx
                    .accounts
                    .presale_associated_token_account
                    .to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.presale_info.to_account_info(),
            },
            &[&[
                b"presale" as &[u8],
                ctx.accounts.presale_info.authority.as_ref(),
                &[bump],
            ][..]],
        ),
        claim_amount,
    )?;

    user_info.buy_token_amount = 0;
    user_info.claim_time = cur_timestamp;
    msg!("All claimed presale tokens transferred successfully.");

    Ok(())
}
