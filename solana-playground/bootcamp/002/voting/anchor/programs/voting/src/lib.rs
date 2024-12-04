#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(ctx:  Context<InitializePoll>, poll_id: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: SignerInfo<'info>,
    #[account(init, payer = payer, space = 8 + Poll::INIT_SPACE, seeds = [poll_id.to_le_bytes().as_ref()])]
    pub poll: Account<'info, Poll>,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    #[max_len(100)]
    pub question: String,
    #[max_len(200)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}