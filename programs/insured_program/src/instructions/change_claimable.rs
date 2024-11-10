use anchor_lang::prelude::*;

use crate::state::{Policy, User};

#[derive(Accounts)]
pub struct ChangeClaimable<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds= [
         b"user_account", 
         policy_pda.key().as_ref()
        ],
        bump
    )]
    pub user_pda: Account<'info, User>,
    #[account(
        seeds = [b"init_policy", policy_pda.name.as_bytes()],
        bump 
    )]
    pub policy_pda: Account<'info, Policy>,
    pub system_program: Program<'info, System>
}

impl<'info> ChangeClaimable<'info> {
    pub fn change_claimable(&mut self)-> Result<()> {
        self.user_pda.is_claimable = false;

        Ok(())
    }
}
