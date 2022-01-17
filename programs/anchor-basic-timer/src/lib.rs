use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;

declare_id!("B3C1yMA1ShnJ2zhfdXU4h1tSHFWEqBT9WLPFHMoPpoh4");

#[program]
pub mod anchor_basic_timer {
    use super::*;
    pub fn initialize_timer(ctx: Context<InitializeTimer>, _bump: u8) -> ProgramResult {
        let clock = clock::Clock::get()?;

        let time = clock.unix_timestamp;

        ctx.accounts.timer.initialized_timestamp = time;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeTimer<'info> {
    #[account(init,
        payer = payer,
        seeds = [b"timer", payer.key().as_ref()],
        bump = bump,
        space = 8 + 8)]
    timer: Account<'info, Timer>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>, 
}

#[account]
pub struct Timer{
    initialized_timestamp: i64,   // 8
    start_time: i64,
    stop_time: i64,
}
