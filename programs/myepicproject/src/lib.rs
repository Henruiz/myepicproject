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
        Ok(())
    }
}

// marco that looks to be a class struct for the fnc
#[derive(Accounts)]
pub struct StartStuffOff {}
