use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserInfo {
    pub buy_quote_amount: u64,
    pub buy_token_amount: u64,
    pub buy_time: u64,
    pub claim_time: u64,
}