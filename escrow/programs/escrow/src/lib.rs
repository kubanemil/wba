pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FHpiZH2awuTi1QvbU6XR15AiCfetEk62oA9BWUn2LxaK");

#[program]
pub mod escrow {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_desired_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .send_offered_tokens_to_vault(token_a_offered_amount)?;
        ctx.accounts
            .save_offer(id, token_b_desired_amount, &ctx.bumps)
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        ctx.accounts.send_wanted_tokens_to_maker()?;
        ctx.accounts.withdraw_and_close_vault()
    }
}
