use anchor_lang::prelude::*;


declare_id!("EcXnBimpAGtH1XwNnuGoALZfpF9bonkbcGpRVYyST3ow");




#[program]
pub mod first{
    use super::*;


    pub fn initialise(ctx:Context<initialise>)-> Result<()>{
        ctx.accounts.account.count=1;
        Ok(())

    }


    pub fn add(ctx:Context<add>, num:u32)-> Result<()>{
        ctx.accounts.account.count= ctx.accounts.account.count+num;
        Ok(())
    }

    pub fn sub(ctx: Context<sub>,num:u32)-> Result<()>{
        ctx.accounts.account.count = ctx.accounts.account.count-num;
        Ok(())
    }

    pub fn double(ctx: Context<double>)-> Result<()>{
        ctx.accounts.account.count = (ctx.accounts.account.count) * 2;
        Ok(())
    }

}






#[account]
pub struct DataShape{
    count : u32
}



#[derive(Accounts)]
pub struct initialise<'info>{
    #[account(init,payer=signer,space=8+4)]
    account: Account<'info,DataShape>,
    system_program: Program<'info,System>,
    #[account(mut)]
    signer: Signer<'info>
}



#[derive(Accounts)]
pub struct double<'info>{
    #[account(mut)]
    account: Account<'info,DataShape>,
    #[account(mut)]
    signer: Signer<'info>
}


#[derive(Accounts)]
pub struct add<'info>{
    #[account(mut)]
    account: Account<'info,DataShape>,
    #[account(mut)]
    signer: Signer<'info>
}


#[derive(Accounts)]
pub struct sub<'info>{
    #[account(mut)]
    account: Account<'info,DataShape>,
    #[account(mut)]
    signer: Signer<'info>
}




