use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;

declare_id!("B3C1yMA1ShnJ2zhfdXU4h1tSHFWEqBT9WLPFHMoPpoh4");

#[program]
pub mod anchor_basic_timer {
    use super::*;
    pub fn initialize_timer(ctx: Context<InitializeTimer>, _bump: u8) -> ProgramResult {
        let clock = clock::Clock::get()?;

        ctx.accounts.timer.initialized_timestamp = clock.unix_timestamp;

        Ok(())
    }

    pub fn start_timer(ctx: Context<StartTimer>) -> ProgramResult {
        let clock = clock::Clock::get()?;

        ctx.accounts.timer.start_time = clock.unix_timestamp;
        ctx.accounts.timer.stop_time = 0;
        ctx.accounts.timer.is_running = true;

        Ok(())
    }

    pub fn stop_timer(ctx: Context<StopTimer>) -> ProgramResult {
        let clock = clock::Clock::get()?;

        ctx.accounts.timer.stop_time = clock.unix_timestamp;
        ctx.accounts.timer.is_running = false;

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
        space = 8 + 8 + 8 + 8 + 1)]
    timer: Account<'info, Timer>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>, 
}

#[derive(Accounts)]
pub struct StartTimer<'info> {
    #[account(mut)]
    timer: Account<'info, Timer>,
}

#[derive(Accounts)]
pub struct StopTimer<'info> {
    #[account(mut)]
    timer: Account<'info, Timer>,
}

#[account]
pub struct Timer{
    initialized_timestamp: i64,   // 8
    start_time: i64,              // 8
    stop_time: i64,               // 8
    is_running: bool,             // 1
}
