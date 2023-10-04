// Imports all the necessary types and macros from the anchor_lang crate.
use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use anchor_lang::AccountsClose;

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
        end_datetime:i64) -> Result<()> {
            // Modify the vesting_data account state 
            // with the new current_amount and end_datetime.
            let vesting_data=&mut ctx.accounts.vesting_data;
            vesting_data.current_amount=current_amount;
            vesting_data.end_datetime=end_datetime;
            let clock=Clock::get()?; //Clock can fail
            vesting_data.last_action_datetime=clock.unix_timestamp;
            return Ok(()); 
        }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let clock=Clock::get()?;
        let current_datetime = clock.unix_timestamp;
        let vesting_data=&mut ctx.accounts.vesting_data;
        // This rate is in lamports per second
        let payout_rate = vesting_data.current_amount /
        ((vesting_data.end_datetime - vesting_data.last_action_datetime)as u64);
        // This is in lamports
        let withdrawal_amount = 
        // If this is the last withdrawal, remove everything
        if current_datetime >= vesting_data.end_datetime {vesting_data.current_amount} 
        // If not last withdrawal, then calculate as usual
        else {((current_datetime - vesting_data.last_action_datetime)as u64)* payout_rate};

        // use anchor_lang::system_program::{Transfer, transfer};
        // Transfer funds from escrow account to recipient
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(), 
            Transfer {
                from: ctx.accounts.escrow_account.to_account_info(),
                to: ctx.accounts.recipient.clone(),
            },
            &[&[b"escrow-seeds"]]
        );
        
        transfer(cpi_context, withdrawal_amount)?;
        // Update the current amount of SOL and last action datetime
        let current_amount = vesting_data.current_amount - withdrawal_amount;
        vesting_data.current_amount = current_amount;
        vesting_data.last_action_datetime = current_datetime;
        if current_amount  == 0 {
            ctx.accounts.vesting_data.close(ctx.accounts.recipient.to_account_info())?;
        }
        Ok(())
    }
    pub fn cancel_vesting(ctx: Context<CancelVesting>) -> Result<()>{
        let current_amount = ctx.accounts.vesting_data.current_amount;
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(), 
            Transfer {
                from: ctx.accounts.escrow_account.to_account_info(),
                to: ctx.accounts.depositor.to_account_info(),
            },
            &[&[b"escrow-seeds"]]
        );
        transfer(cpi_context, current_amount)?;
        //Close vesting account and return funds to recipient
        ctx.accounts.vesting_data.close(ctx.accounts.recipient.to_account_info())?;
        Ok(())
    }

    pub fn get_remaining_amount(ctx: Context<GetRemainingAmount>) -> Result<u64>{
        Ok(ctx.accounts.vesting_data.current_amount)
    }
}

// A struct representing the state of a vesting contract.
#[account]
pub struct VestingData {
  current_amount: u64, // The current amount vested.
  end_datetime: i64, // The datetime when the vesting ends.
  last_action_datetime: i64, // The datetime of the last action.
  bump: u8
}

// A struct representing the context of the create_vesting function.
#[derive(Accounts)]
pub struct CreateVesting<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    // An account to hold the VestingData, which is created and paid for by the recipient.
    // space: 8 discriminator + 8 current_amount + 8 end_datetime + 8 last_action_datetime
    pub vesting_data: Account<'info, VestingData>,
    // A reference to the system program, which is used to create the vesting_data account.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info>{
    /// CHECK: The recipient is used to receive funds
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    /// CHECK: The depositor account will be returned rent after closing the vesting data account    
    #[account(mut)]
    pub depositor: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"vesting-data", recipient.key().as_ref(), depositor.key().as_ref()], 
        bump = vesting_data.bump
    )]
    pub vesting_data: Account<'info, VestingData>,
    #[account(mut)]
    pub escrow_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetRemainingAmount<'info>{
    pub vesting_data: Account<'info, VestingData>,
}

#[derive(Accounts)]
pub struct CancelVesting<'info>{
    /// CHECK: We need to use recipient to derive the PDA
    pub recipient: AccountInfo<'info>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vesting-data", recipient.key().as_ref(), depositor.key().as_ref()], 
        bump = vesting_data.bump
    )]
    pub vesting_data: Account<'info, VestingData>,
    #[account(mut)]
    pub escrow_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}