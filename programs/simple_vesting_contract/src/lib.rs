// Imports all the necessary types and macros from the anchor_lang crate.
use anchor_lang::prelude::*;

// A unique ID for the program, also known as programId in Solana, 
// which is a public key that identifies your program on the blockchain.
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// Declare a new program named simple_vesting_contract.
#[program]
pub mod simple_vesting_contract {
    // Import everything from the surrounding (super) module.
    use super::*;

    // The function that creates a vesting contract 
    // with specified current_amount and end_datetime.
  	pub fn create_vesting(ctx: Context<CreateVesting>, 
        current_amount:u64, 
        end_datetime:u64) -> Result<()> {
            // Modify the vesting_data account state 
            // with the new current_amount and end_datetime.
            let vesting_data=&mut ctx.accounts.vesting_data;
            vesting_data.current_amount=current_amount;
            vesting_data.end_datetime=end_datetime;
            let clock=Clock::get()?; //Clock can fail
            vesting_data.last_action_datetime=clock.unix_timestamp;
            return Ok(()); 
        }
}

// A struct representing the state of a vesting contract.
#[account]
pub struct VestingData {
  current_amount: u64, // The current amount vested.
  end_datetime: u64, // The datetime when the vesting ends.
  last_action_datetime: u64 // The datetime of the last action.
}

// A struct representing the context of the create_vesting function.
#[derive(Accounts)]
pub struct CreateVesting<'info> {
    // A mutable reference to the user's account who signs the transaction.
    #[account(mut)]
    pub user: Signer<'info>,
    // An account to hold the VestingData, which is created and paid for by the user.
    // space: 8 discriminator + 8 current_amount + 8 end_datetime + 8 last_action_datetime
    #[account(
        init,
        payer = user,
        space = 8 + 8 + 8 + 8, // The size of the account to be created. It's a sum of the sizes of the fields in VestingData.
        seeds = [b"vesting-data", user.key().as_ref()], bump
    )]
    pub vesting_data: Account<'info, VestingData>,
    // A reference to the system program, which is used to create the vesting_data account.
    pub system_program: Program<'info, System>,
}