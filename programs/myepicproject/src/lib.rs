// import anchor tools we need
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

// program id
declare_id!("BH4p7HLhjWqiChFj9Bz4wyvzfXE5mRKUrZ8nqCmfXuwe");

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

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // initialize counter
        base_account.total_gifs = 0;
        Ok(())
    }

    // adding function to add gif to the account
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;
        // reference to user
        let user = &mut ctx.accounts.user;

        // Struct for items
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        // adding gif list and num of gifs to base_account
        base_account.gif_list.push(item);
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
    #[account(mut)]
    pub user: Signer<'info>,
}

// creating custom data obj to hold our info for us
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}
/// what we are storing in the account obj
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}
