use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::{
    state::policy::Policy,
    errors::Errors            
};


#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitPolicy<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init,
        payer = owner,
        space = Policy::INIT_SPACE + 8,
        seeds = [b"init_policy", name.as_bytes()],
        bump
    )]
    pub policy_pda: Account<'info, Policy>,
    #[account(
        mut,
        seeds = [b"policy_vault", policy_pda.key().as_ref()],
        bump
    )]
    pub policy_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}


impl<'info> InitPolicy<'info> {
    pub fn init_policy(&mut self, name: String, claim_amount: u64) -> Result<()> {
        require!(name.len() <= 20, Errors::NameBiggerThanExpected);
        self.policy_pda.set_inner(Policy{
            claim_amount,
            name
        });

        let cpi_accounts = Transfer{
            from: self.owner.to_account_info(),
            to: self.policy_vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        transfer(cpi_context, claim_amount)
    }
}

