use anchor_lang::prelude::*;
use crate::state::{
    policy::Policy,
    user::User
};

#[derive(Accounts)]
pub struct OptUser<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"init_policy", policy_pda.name.as_bytes()],
        bump 
    )]
    pub policy_pda: Account<'info, Policy>,
    #[account(
        mut,
        seeds = [b"policy_vault", policy_pda.key().as_ref()],
        bump
    )]  
    pub policy_vault: SystemAccount<'info>,
    #[account(
        init, 
        payer = user,
        seeds= [b"user_account", policy_pda.key().as_ref()],
        bump,
        space =   User::INIT_SPACE + 8
    )]
    pub user_account: Account<'info, User>,
    pub system_program: Program<'info, System>
}


impl<'info> OptUser<'info> {
    pub fn create_user(&mut self) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        self.user_account.set_inner(User { created_at: timestamp, is_claimable: false });
        
        Ok(())
    }
}