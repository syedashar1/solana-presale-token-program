use anchor_lang::{prelude::*, system_program};

use crate::state::PresaleInfo;

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(
        seeds = [
            b"presale",
            presale_info.authority.as_ref()
        ],
        bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    /// CHECK:
    #[account(
        mut,
        seeds = [b"vault"],
        bump
    )]
    pub presale_vault: AccountInfo<'info>,

    #[account(
        mut,
        constraint = admin.key() == presale_info.authority
    )]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64, bump: u8) -> Result<()> {
    msg!(
        "Vault: {:?} Send Amount {:?}",
        ctx.accounts.presale_vault.to_account_info().lamports(),
        amount
    );
    system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.presale_vault.to_account_info(),
                to: ctx.accounts.admin.to_account_info(),
            },
            &[&[b"vault" as &[u8], &[bump]][..]],
        ),
        amount,
    )?;

    Ok(())
}
