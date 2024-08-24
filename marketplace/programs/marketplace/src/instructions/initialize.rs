use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::error::ErrorCode;
use crate::state::Marketplace;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,

    #[account(init, space=Marketplace::INIT_SPACE, payer=admin, seeds=[b"marketplace", name.as_bytes()], bump)]
    marketplace: Account<'info, Marketplace>,

    #[account(init, seeds=[b"rewards", marketplace.key().as_ref()], payer=admin, bump, mint::decimals=6, mint::authority=marketplace)]
    rewards_mint: InterfaceAccount<'info, Mint>,

    #[account(seeds=[b"treasury", marketplace.key().as_ref()], bump)]
    treasury: SystemAccount<'info>,

    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        require!(!name.is_empty() && name.len() < 33, ErrorCode::TooLongName);

        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            rewards_bump: bumps.rewards_mint,
            treasury_bump: bumps.treasury,
            name,
        });

        Ok(())
    }
}
