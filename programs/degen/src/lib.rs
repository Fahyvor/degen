use anchor_lang::prelude::*;

declare_id!("AZbrm4ztq6iyUwZtXSvt5mjJWpq6QGMLsFeccyYDKEDL");

#[program]
pub mod degen {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
