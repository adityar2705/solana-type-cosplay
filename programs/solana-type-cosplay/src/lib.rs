use anchor_lang::prelude::*;
use borsh::{BorshDeserialize,BorshSerialize};
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::create_account;

declare_id!("5Cc1pj5VU6TFMMz3o9AX1RRtgCFeo65Q1AJZ61GRFwJ5");

#[program]
pub mod solana_type_cosplay {
    use super::*;

    //initialize the admin account
    pub fn initialize_admin(ctx: Context<Initialize>) -> Result<()> {
        //space for the public key
        let space = 32;

        //calculating the rent for the new account
        let lamports = Rent::get()?.minimum_balance(space as usize);

        //create account instruction
        let inst = create_account(&ctx.accounts.payer.key(), &ctx.accounts.new_account.key(), lamports, space, &ctx.program_id);

        //invoke CPI with the system program to create the account at the new_account address using the create_account instruction
        invoke(&inst, 
        &[  
            //all the accounts that the create_account instruction needs
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.new_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ])?;

        //getting the account -> so that we can use it and set the data
        let mut account = AdminConfig::try_from_slice(&ctx.accounts.new_account.data.borrow()).unwrap();
        account.admin = ctx.accounts.payer.key();

        //serialize the account -> important
        account.serialize(&mut *ctx.accounts.new_account.data.borrow_mut())?;
        msg!("Admin: {}", account.admin.to_string());

        Ok(())
    }

    //function to initialize the user account
    pub fn initialize_user(ctx: Context<Initialize>)->Result<()>{
        //getting the space and rent
        let space = 32;
        let lamports = Rent::get()?.minimum_balance(space as usize);

        //create account instruction
        let inst = create_account(&ctx.accounts.payer.key(), &ctx.accounts.new_account.key(), lamports, space, &ctx.program_id);

        //invoke CPI with the system program to create the account at the new_account address using the create_account instruction
        invoke(&inst, 
            &[  
                //all the accounts that the create_account instruction needs
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.new_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
        ])?;
        
        //getting the new user account
        let mut account = User::try_from_slice(&ctx.accounts.new_account.data.borrow()).unwrap();
        account.user = ctx.accounts.payer.key();

        //serialize the user account -> important
        account.serialize(&mut *ctx.accounts.new_account.data.borrow_mut())?;

        msg!("User: {}", account.user.to_string());
        Ok(())
    }

    //function to update the admin
    pub fn update_admin(ctx : Context<UpdateAdmin>)->Result<()>{
        //lets get the particular account
        let mut account = AdminConfig::try_from_slice(&ctx.accounts.admin_config.data.borrow()).unwrap();

        //check whether the current admin set is passed into the instruction as the admin field
        if ctx.accounts.admin.key() != account.admin {
            return Err(ProgramError::InvalidAccountData.into());
        }

        //set the new admin and serialize the account
        account.admin = ctx.accounts.new_admin.key();
        account.serialize(&mut *ctx.accounts.admin_config.data.borrow_mut())?;

        msg!("New Admin: {}", account.admin.to_string());

        Ok(())
    }

}

//struct for the initialize user/admin config instruction
#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub new_account : Signer<'info>,
    #[account(mut)]

    //the account that will pay for the creation of the new account
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//struct for the update admin instruction -> only the previous admin should be able to perform it
#[derive(Accounts)]
pub struct UpdateAdmin<'info>{
    #[account(mut)]
    /// CHECK:
    admin_config : AccountInfo<'info>,
    new_admin : SystemAccount<'info>,
    admin : Signer<'info>
}

//admin config struct
#[derive(BorshSerialize,BorshDeserialize)]
pub struct AdminConfig{
    admin : Pubkey
}

//user struct 
#[derive(BorshSerialize,BorshDeserialize)]
pub struct User{
    user : Pubkey
}
