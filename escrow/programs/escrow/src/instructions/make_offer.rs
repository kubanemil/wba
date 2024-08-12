use crate::{Offer, ANCHOR_DISCRIMINATOR};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
        )]
    pub offer: Account<'info, Offer>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
        )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

impl<'info> MakeOffer<'info> {
    pub fn send_offered_tokens_to_vault(&self, token_a_offered_amount: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.maker_token_account_a.to_account_info(),
            mint: self.token_mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };
        let cpi_context = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        transfer_checked(
            cpi_context,
            token_a_offered_amount,
            self.token_mint_a.decimals,
        )
    }

    pub fn save_offer(
        &mut self,
        id: u64,
        token_b_desired_amount: u64,
        bumps: &MakeOfferBumps,
    ) -> Result<()> {
        self.offer.set_inner(Offer {
            id,
            maker: self.maker.key(),
            token_a: self.token_mint_a.key(),
            token_b: self.token_mint_b.key(),
            token_b_desired_amount,
            bump: bumps.offer,
        });
        Ok(())
    }
}
