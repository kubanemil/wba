use crate::state::Bet;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    pub house: UncheckedAccount<'info>,

    #[account(mut, seeds=[b"vault", house.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,

    #[account(init, payer=player, space=Bet::LEN, seeds=[b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()], bump)]
    pub bet: Account<'info, Bet>,

    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn create_bet(
        &mut self,
        bumps: &PlaceBetBumps,
        seed: u64,
        amount: u64,
        roll: u8,
    ) -> Result<()> {
        self.bet.set_inner(Bet {
            player: self.player.key(),
            seed,
            amount,
            slot: Clock::get()?.slot,
            roll,
            bump: bumps.bet,
        });
        Ok(())
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.player.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(cpi, amount)
    }
}
