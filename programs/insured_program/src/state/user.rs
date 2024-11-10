use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub created_at: i64,
    pub is_claimable: bool
}