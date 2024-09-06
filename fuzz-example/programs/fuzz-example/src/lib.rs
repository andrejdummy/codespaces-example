use anchor_lang::prelude::*;

declare_id!("4YiJYdQLhSfA9FvoXhy2se9eQtUMZ5PT1tSDR64rMc8k");

#[program]
pub mod fuzz_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub signer: Signer<'info>,
}
