// import anchor tools we need
use anchor_lang::prelude::*;

// program id
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// start of macros
#[program]
pub mod myepicproject {
    use super::*;
    // function start stuff off
    // takes in context and outputs ProgramResult
    // doesnt do anything at all but soon we can add logic to it
    // pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    //     Ok(())
    // }

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        // reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // initialize counter
        base_account.total_gifs = 0;
        Ok(())
    }

    // adding function to add gif to the account
    pub fn add_gif(ctx: Context<AddGif>) -> Result<()> {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs += 1;
        Ok(())
    }
}

// marco that looks to be a class struct for the fnc
// set variables
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 10000)] // how account will be initialize
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// macro to set variables for what the addgif context will store
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

/// what we are storing in the account obj
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}
