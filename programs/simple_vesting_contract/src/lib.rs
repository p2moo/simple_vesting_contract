use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod simple_vesting_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
  
  	pub fn create_vesting()
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct VestingData {
  current_amount: u64,
  end_datetime: u64,
  last_action_datetime: u64
}

#[derive(Accounts)]
pub struct CreateVesting<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // space: 8 discriminator + 8 current_amount + 8 end_datetime + 8 last_action_datetime
    #[account(
        init,
        payer = user,
        space = 8 + 8 + 8 + 8, seeds = [b"vesting-data", user.key().as_ref()], bump
    )]
    pub vesting_data: Account<'info, VestingData>,
    pub system_program: Program<'info, System>,
}