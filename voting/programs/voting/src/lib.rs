use anchor_lang::prelude::{borsh::{BorshDeserialize, BorshSerialize}, *};

declare_id!("6u6rSmVt8AqXFH5hjgKPwUViCn6DjrM4ZFA4n4rRgisE");

#[error_code]
pub enum VotingError{
    #[msg("Overflow")]
    Overflow,
    #[msg("Underflow")]
    Underflow,
    #[msg("Unauthorized")]
    Unauthorized
}

#[derive(AnchorDeserialize,AnchorSerialize,Clone,PartialEq,Eq)]
pub enum VotingType{
    UPVOTE,
    DOWNVOTE
}

#[program]
pub mod Voting{
    use super::*;
    
    pub fn initialise(ctx: Context<Initialise>, title: String) -> Result<()>{
        let account = &mut ctx.accounts.voting_account;
        account.authority = ctx.accounts.signer.key();
        account.title = title;
        account.votes = 0;
        Ok(())
    }
    
    pub fn vote(ctx: Context<UpdateVote>, vote: VotingType) -> Result<()>{


        let account = &mut ctx.accounts.voting_account;
        require_keys_eq!(account.authority,ctx.accounts.signer.key(),VotingError::Unauthorized);
        match vote{
            VotingType::UPVOTE => {
                account.votes = account.votes.checked_add(1).ok_or(VotingError::Overflow)?;
            }
            VotingType::DOWNVOTE => {
                account.votes = account.votes.checked_sub(1).ok_or(VotingError::Underflow)?;
            }
        }
        Ok(())
    }

    pub fn result(ctx: Context<ResultVote>) -> Result<bool>{
        let account = &mut ctx.accounts.voting_account;
        require_keys_eq!(account.authority,ctx.accounts.signer.key(),VotingError::Unauthorized);
        let passed = account.votes > 0;
        Ok(passed)
    }
}

#[account]
pub struct VotingAccount{
    pub authority: Pubkey,  // 32 bytes
    pub title: String,      // 4 + length bytes (variable)
    pub votes: i32          // 4 bytes
}

#[derive(Accounts)]
pub struct Initialise<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + (4 + 100) + 4, // discriminator + pubkey + (string_len + max_string) + i32
        seeds = [b"poll", signer.key().as_ref()],
        bump
    )]
    pub voting_account: Account<'info, VotingAccount>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateVote<'info>{
    #[account(mut)] // Add mut since you're modifying votes
    pub voting_account: Account<'info, VotingAccount>,
    pub signer: Signer<'info>
}

#[derive(Accounts)]
pub struct ResultVote<'info>{
    #[account(mut, close = signer)]
    pub voting_account: Account<'info, VotingAccount>,
    #[account(mut)]
    pub signer: Signer<'info>
}