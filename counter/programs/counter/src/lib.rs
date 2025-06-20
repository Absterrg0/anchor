use anchor_lang::prelude::*;



declare_id!("8B63XKxqi6gdUtCyeyzUqLRhkyggYnQZp85EqCBmRh8h");



#[error_code]
pub enum CounterError{
    #[msg("Arithematic Overflow")]
    Overflow,
    #[msg("Arithematic Underflow")]
    Underflow
    
}


#[program]
pub mod Counter{
    
    use super::*;

    pub fn initialise(ctx:Context<Initialise>,value:u32)->Result<()>{

        ctx.accounts.data_account.count = value;
        Ok(())
    }

    pub fn increment(ctx:Context<Increment>,value:u32)-> Result<()>{

        ctx.accounts.data_account.count = ctx.accounts.data_account.count.checked_add(value).ok_or(CounterError::Overflow)?;
        Ok(())
    }

    pub fn decrement(ctx:Context<Decrement>,value:u32)->Result<()>{

        ctx.accounts.data_account.count = ctx.accounts.data_account.count.checked_sub(value).ok_or(CounterError::Underflow)?;
        Ok(())
    }
    

}



#[account]
pub struct DataOnChain{
    count:u32
}



#[derive(Accounts)]
pub struct Initialise<'info>{
    #[account(mut)]
    signer: Signer<'info>,
    #[account(init, payer=signer, space=8+4)]
    data_account: Account<'info,DataOnChain>,
    system_program: Program<'info,System>
}



#[derive(Accounts)]
pub struct Increment<'info>{
    #[account(mut)]
    data_account: Account<'info,DataOnChain>
}



#[derive(Accounts)]
pub struct Decrement<'info>{
    #[account(mut)]
    data_account: Account<'info,DataOnChain>
}

