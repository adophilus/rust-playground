use anchor_lang::prelude::*;

declare_id!("kkitw3Ba3maZo5R3UZAqXR7EnLvDBQmgNd6D3HsTZrj");

#[program]
pub mod staking_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
