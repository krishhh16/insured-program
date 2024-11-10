use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::{
    state::{Policy, User},
    errors::Errors
};

#[derive(Accounts)]
pub struct Claim<'info> {
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
    pub user_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        seeds = [b"policy_vault", policy_pda.key().as_ref()],
        bump
    )]  
    pub policy_vault: SystemAccount<'info>,
}

impl<'info> Claim<'info> {
    pub fn claim_reward(&mut self, bumps: &ClaimBumps) -> Result<()> {

        require!(!self.user_pda.is_claimable, Errors::NotClaimable);
        let policy_vault: &[u8; 12]      = b"policy_vault";
        let seeds  = &[
           policy_vault,
            self.policy_pda.to_account_info().key.as_ref(),
        &[bumps.policy_vault]
        ];

        let signer_seeds = &[&seeds[..]];

        let system_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from: self.policy_vault.to_account_info(),
            to: self.user_account.to_account_info()
        };

        let cpi_context = CpiContext::new_with_signer(system_program, cpi_accounts, signer_seeds);
        

        transfer(cpi_context, self.policy_pda.claim_amount)
    }
}
