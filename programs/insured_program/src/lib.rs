use anchor_lang::prelude::*;

declare_id!("8kSerJYf5iDfnmKbMciuquwyuMpRTj372hgJTtnrz6cC");

#[program]
pub mod insured_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}