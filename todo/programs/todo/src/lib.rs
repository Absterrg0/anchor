
use anchor_lang::prelude::*;




declare_id!("HDksv8e2YZVDVbvRdhYRNDU4SV9KQEsWsyZPYLmjqokg");





#[error_code]
pub enum TodoErrors{
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Todo does not exist")]
    BadRequest
}

#[program]
pub mod Todo{

    use super::*;


    pub fn init(ctx: Context<Initialise>)->Result<()>{

        let account = &mut  ctx.accounts.todoAccount;

        account.authority = ctx.accounts.signer.key();
        account.todos = Vec::new();

        Ok(())
    }

    pub fn add_todo(ctx: Context<UpdateTodo>,description:String,)->Result<()>{

        let account = &mut ctx.accounts.todoAccount;

        require_keys_eq!(account.authority,ctx.accounts.signer.key(),TodoErrors::Unauthorized);

        account.todos.push(TodoItem { description: (description), is_complete: (false) });


        Ok(())
    }
    

    pub fn mark_todo_as_completed(ctx: Context<UpdateTodo>,index:u32)->Result<()>{

        let account = &mut ctx.accounts.todoAccount;

        require_keys_eq!(account.authority,ctx.accounts.signer.key(),TodoErrors::Unauthorized);

        let task = account.todos.get_mut(index as usize).ok_or(TodoErrors::BadRequest)?;

        task.is_complete = !task.is_complete;
        Ok(())
    }

    pub fn delete_todo(ctx: Context<UpdateTodo>,index:u32)->Result<()>{

        let account = &mut ctx.accounts.todoAccount;

        require_keys_eq!(account.authority,ctx.accounts.signer.key(),TodoErrors::BadRequest);

        account.todos.remove(index as usize);

        Ok(())
        
    }
}





#[account]
pub struct OnChainData{
    pub authority : Pubkey,
    pub todos: Vec<TodoItem>
}


#[derive(AnchorSerialize,AnchorDeserialize,Clone)]
pub struct TodoItem{
    pub description: String,
    pub is_complete: bool
}




#[derive(Accounts)]
pub struct Initialise<'info>{
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        payer=signer,
        space = 8 + 32 + (4  * 100 + (4 * 280 + 1)),
        seeds = [b"todo",signer.key().as_ref()],
        bump
    )]
    todoAccount: Account<'info,OnChainData>,
    system_program: Program<'info,System>
} 



#[derive(Accounts)]
pub struct UpdateTodo<'info>{
    signer: Signer<'info>,
    todoAccount: Account<'info,OnChainData>
}



