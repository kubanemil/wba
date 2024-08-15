pub mod constants;
pub mod error;
pub mod helpers;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("646fEJR9Q4LuQkvzmohaecDB2fnRQjwLTMHYZ3DZaey8");

#[program]
pub mod amm {
    use super::*;

    pub fn init_pool(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, fee, authority)
    }

    // provide liquidity to receive LP tokens
    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
        max_x: u64,
        max_y: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y, expiration)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
        min_x: u64,
        min_y: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts.withdraw(amount, min_x, min_y, expiration)
    }

    pub fn swap(
        ctx: Context<Swap>,
        is_token_x: bool,
        amount: u64,
        min_receive: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts
            .swap(is_token_x, amount, min_receive, expiration)
    }
}
