use anchor_lang::{prelude::*, solana_program::{ed25519_program, sysvar::instructions::load_instruction_at_checked}, system_program::{transfer, Transfer}};
use crate::state::Bet;
use crate::error::ErrorCode;


#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,

    pub player: UncheckedAccount<'info>,

    #[account(mut, seeds=[b"vault", house.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,
    
    #[account(mut, seeds=[b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()], bump=bet.bump)]
    pub bet: Account<'info, Bet>,

    #[account(address=anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instruction_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        let ix = load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;
        require_keys_eq!(ix.program_id, ed25519_program::ID, ErrorCode::Ed25519Program);
        require_eq!(ix.accounts.len(), 0, ErrorCode::Ed25519Account);

        let signature = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signature.len(), 0, ErrorCode::Ed25519DataLength);

        let signature = &signature[0];

        require!(signature.is_verifiable);

        require_keys_eq!(signature.public_key.ok_or(ErrorCode::Ed25519Pubkey)?, self.house.key(), ErrorCode::Ed25519Pubkey);

        require!(&signature.message.ok_or(ErrorCode::Ed25519Signature)?.eq(sig), ErrorCode::Ed25519Signature);
        todo!()
        // Ok(())
    }


    pub fn resolve_bet(&mut self, bumps: &ResolveBet) -> Result<()> {
        Ok(())
    }
}
