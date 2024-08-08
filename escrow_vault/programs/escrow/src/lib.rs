use anchor_lang::prelude::*;

mod instructions;
use instructions::*;
mod state;
use state::*;

declare_id!("2QJDtP79CcHBw1EeG5hTy2N8n8ZCbL7gcg98HmpebDGG");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64, receive: u64) -> Result<()> {
        ctx.accounts.create_escrow(seed, receive, ctx.bumps.escrow)?;
        ctx.accounts.deposit_to_vault(amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.transfer_to_maker()?;
        ctx.accounts.withdraw_and_close()
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close()
    }
}