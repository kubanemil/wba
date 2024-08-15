use crate::error::ErrorCode;
use crate::Offer;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, Token2022, TokenAccount,
        TransferChecked,
    },
};

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    // You can use AccountInfo - it doesn't make any validation or checks for account
    // Or UncheckedAccount (recommended). It's possible to add custom constraints on them.
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>, // checks if account is owned by System Program.

    pub token_a: InterfaceAccount<'info, Mint>,
    pub token_b: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = token_a,
        associated_token::authority = taker,
        associated_token::token_program= token_program
    )]
    pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_b,
        associated_token::authority = taker,
        associated_token::token_program= token_program
    )]
    pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = token_b,
        associated_token::authority = maker,
        associated_token::token_program= token_program
    )]
    // Box<> allocates account to heap (cheaper) - if accounts are too large
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        close=maker, // will close account at the end of instruction.
        has_one=maker,
        has_one=token_a @ ErrorCode::NoTokenAError,  // offer.token_a.key() == TakeOffer.token_a.key()
        // `address=other_program::ID` to make sure that account has specific address
        // `owner=owner_pubkey` - make sures owner of account is specific pubkey/ID
        // use `constraint=bolean_expression` for custom constraint

        // realloc = NEW_SPACE_IN_BYTES, // to reallocate space (if less space, rent will returned)
        // realloc::payer = maker,
        // realloc::zero = false,
        has_one=token_b,
        seeds=[b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
        bump=offer.bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = token_a,
        associated_token::authority = offer,
        associated_token::token_program= token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // 3 anchor program types given by Anchor: System, Token, AssociatedToken
    // If other program, you need to import program type
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>, // Checks that account is executable
}

impl<'info> TakeOffer<'info> {
    pub fn send_wanted_tokens_to_maker(&self) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            mint: self.token_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        transfer_checked(
            cpi_ctx,
            self.offer.token_b_desired_amount,
            self.token_b.decimals,
        )
    }

    pub fn withdraw_and_close_vault(&self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"offer",
            self.maker.to_account_info().key.as_ref(),
            &self.offer.id.to_le_bytes()[..],
            &[self.offer.bump],
        ]];
        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.token_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.offer.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            &signer_seeds,
        );
        transfer_checked(cpi_context, self.vault.amount, self.token_a.decimals)?;
        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.offer.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            &signer_seeds,
        );
        close_account(cpi_context)
    }
}
