use anchor_lang::{prelude::*};

declare_id!("FKonoaK2EEFhD2arreKsnDfjFP9Xk7V5j2jsXZa1GHRq");

#[program]
pub mod solbank {
    
    use super::*;
    pub fn create(ctx: Context<Setup>) -> Result<()> {

        let solbank = &mut ctx.accounts.client;
        solbank.amount = 10;
        solbank.owner = *ctx.accounts.user.key;
        solbank.timestamp = Clock::get().unwrap().unix_timestamp;
        Ok(())

    }
    pub fn deposit(ctx: Context<Deposit>,amount:u64) -> Result<()> {
        let solbank = &mut ctx.accounts.client;
        let user = &ctx.accounts.user;

        if solbank.owner != *user.key{
            return Err(ProgramError::IllegalOwner.into());
        }

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &user.key,
            &solbank.key(),
            amount
        );
        let _ = anchor_lang::solana_program::program::invoke(
            &ix,
            &[user.to_account_info(),solbank.to_account_info()]
        );
        solbank.amount += amount;
        Ok(())

    }
    
    pub fn withdraw(ctx: Context<Withdraw>,amount: u64) -> Result<()>{
        let solbank = &mut ctx.accounts.client;
        let user = &ctx.accounts.user;
        if solbank.owner != *user.key{
            return Err(ProgramError::IllegalOwner.into());
        }
        
        let rent = Rent::get()?.minimum_balance(solbank.to_account_info().data_len());

        if **solbank.to_account_info().lamports.borrow() as u64 - rent < amount{
            return Err(ProgramError::InsufficientFunds.into())
        };
        **solbank.to_account_info().try_borrow_mut_lamports()? -=amount;
        **user.to_account_info().try_borrow_mut_lamports()? +=amount;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init,payer=user,space = 9000,seeds = [b"vault".as_ref(),user.key().as_ref()],bump)]
    pub client: Account<'info, Client>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub client: Account<'info, Client>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub client: Account<'info, Client>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Balance<'info> {
    #[account(mut)]
    pub client: Account<'info, Client>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct Client {
    amount :u64,
    owner : Pubkey,
    timestamp: i64
}
