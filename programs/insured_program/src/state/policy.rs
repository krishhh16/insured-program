use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Policy {
    pub claim_amount: u64,
    #[max_len(20)]
    pub name: String
}