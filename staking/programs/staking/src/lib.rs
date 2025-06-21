
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::{Mint, Token, TokenAccount},
    token_interface::{transfer_checked,TransferChecked},
    token::{mint_to,MintTo}
};



declare_id!("GJRb1k6wCFTvMcNG9PA3mUQULZnqPU5GfrKNtm4H4SXx");


#[error_code]
pub enum CustomError{
    #[msg("Insufficient Balance")]
    InsufBalance,
    #[msg("Overflow")]
    Overflow,
    #[msg("Underflow")]
    Underflow
}


#[program]
pub mod staking{
    use super::*;
    pub fn initialise(ctx: Context<Initialise>)->Result<()>{

        let global_state = &mut ctx.accounts.global_state;
        global_state.authority = ctx.accounts.signer.key();
        global_state.bump = ctx.bumps.global_state;
        global_state.reward_mint = ctx.accounts.reward_mint.key();
        Ok(())
    }

    pub fn stake(ctx:Context<Stake>,amount:u64)->Result<()>{

        let clock = Clock::get()?;
        let user_stake = &mut ctx.accounts.user_stake_account;

        user_stake.authority=ctx.accounts.signer.key();
        user_stake.staking_mint = ctx.accounts.user_token_mint.key();
        user_stake.staked_amount = user_stake.staked_amount.checked_add(amount).ok_or(CustomError::Overflow)?;
        user_stake.start_slot = clock.slot;
        user_stake.last_claimed_slot = clock.slot;


        let transfer_accounts = TransferChecked{
            from:ctx.accounts.user_token_ata.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority:ctx.accounts.signer.to_account_info(),
            mint : ctx.accounts.user_token_mint.to_account_info()
        };

        let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_accounts);
        
        transfer_checked(cpi_context, amount,ctx.accounts.user_token_mint.decimals)?;
        Ok(())
    }

    pub fn unstake(ctx:Context<Unstake>)->Result<()>{

        let user_stake = &mut ctx.accounts.user_stake_account;

        let amount = user_stake.staked_amount;

        let signer_key = ctx.accounts.signer.key();
        let staking_mint_key = user_stake.staking_mint.key();
        let bump_array = [ctx.bumps.vault];
        let seeds = &[b"user_account", signer_key.as_ref(), staking_mint_key.as_ref(), &bump_array];

        let transfer_accounts = TransferChecked{
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user_token_ata.to_account_info(),
            mint: ctx.accounts.user_token_mint.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[seeds];
        let cpi_context = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), transfer_accounts, signer_seeds);


        transfer_checked(cpi_context, amount, ctx.accounts.user_token_mint.decimals)?;


        
        Ok(())
    }
    pub fn claim_rewards(ctx: Context<ClaimRewards>)->Result<()>{

        let clock = Clock::get()?;

        let user_stake = &mut ctx.accounts.user_stake_account;

        let total_blocks = clock.slot - user_stake.last_claimed_slot;

        let reward = total_blocks
        .checked_mul(2)
        .and_then(|v| v.checked_mul(user_stake.staked_amount))
        .ok_or(CustomError::Overflow)?;

        user_stake.last_claimed_slot = clock.slot;

        let mint_accounts = MintTo{
            to: ctx.accounts.user_reward_ata.to_account_info(),
            mint:ctx.accounts.reward_mint.to_account_info(),
            authority:ctx.accounts.reward_authority.to_account_info(),
        };
        let seeds: &[&[u8]] = &[b"mint_authority",&[ctx.bumps.reward_authority]];
        let signer = &[seeds];
        let cpi_context = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), mint_accounts,signer ); 
        mint_to(cpi_context, reward)?;


        Ok(())
    }
}





#[account]
pub struct GlobalState{
    authority: Pubkey,
    reward_mint: Pubkey,
    bump: u8,
}



#[account]
pub struct UserStake{
    pub authority:Pubkey,
    pub staking_mint: Pubkey,
    pub staked_amount: u64,
    pub start_slot: u64,
    pub last_claimed_slot: u64
}




#[derive(Accounts)]
pub struct Initialise<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    pub system_program:Program<'info,System>,
    pub token_program: Program<'info,Token>,
    #[account(
        init,
        payer=signer,
        space = 32 + 32 + 1 + 8 + 1,
        seeds=[b"state",signer.key().as_ref()],
        bump
    )]
    pub global_state: Account<'info,GlobalState>,
    pub reward_mint: Account<'info,Mint>
}



#[derive(Accounts)]
pub struct Stake<'info>{

    #[account(mut)]
    pub signer:Signer<'info>,
    pub system_program: Program<'info,System>,
    pub user_token_mint: Account<'info,Mint>,
    pub user_token_ata: Account<'info,TokenAccount>,
    pub token_program: Program<'info,Token>,
    #[account(
        init_if_needed,
        payer=signer,
        token::mint = user_token_mint,
        token::authority = vault_authority
    )]
    pub vault: Account<'info,TokenAccount>,
    #[account(
        seeds=[b"authority",user_token_mint.key().as_ref()],
        bump
    )]
    ///CHECK: This is to just sign the transactions through the vault
    pub vault_authority: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer=signer,
        space = 32 + 32 + 8 + 8 + 8 +8 + 1,
        seeds = [b"user_account",signer.key().as_ref(),user_token_mint.key().as_ref()],
        bump,
    )]
    pub user_stake_account: Account<'info,UserStake>,
}





#[derive(Accounts)]
pub struct Unstake<'info>{

    #[account(mut)]
    signer:Signer<'info>,
    pub system_program:Program<'info,System>,
    pub token_program: Program<'info,Token>,
    #[account(
        seeds = [b"vault",user_stake_account.staking_mint.as_ref()],
        bump
    )]
    pub vault: Account<'info,TokenAccount>,
    /// CHECK: This is a PDA derived from [b"vault", mint]. It only signs, never read directly.
    #[account(
        seeds=[b"authority",user_stake_account.staking_mint.as_ref()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,
    #[account(
        mut,
        close = signer,
        seeds=[b"user_account",signer.key().as_ref(),user_stake_account.staking_mint.as_ref()],
        bump 
    )]
    pub user_stake_account: Account<'info,UserStake>,
    #[account(
        seeds=[b"state",signer.key().as_ref()],
        bump = global_state.bump
    )]
    pub global_state: Account<'info,GlobalState>,
    pub user_token_mint: Account<'info,Mint>,
    pub user_token_ata: Account<'info,TokenAccount>,
}



#[derive(Accounts)]
pub struct ClaimRewards<'info>{

    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds=[b"user_account",signer.key().as_ref(),user_stake_account.staking_mint.as_ref()],
        bump 
    )]
    pub user_stake_account: Account<'info,UserStake>,
    #[account(
        seeds = [b"state",signer.key().as_ref()],
        bump
    )]
    pub global_state: Account<'info,GlobalState>,
    #[account(mut)]
    pub reward_mint: Account<'info,Mint>,
    #[account(
        
        init_if_needed,
        payer= signer,
        associated_token::mint = reward_mint,
        associated_token::authority = signer
    )]
    pub user_reward_ata: Account<'info,TokenAccount>,
    /// CHECK: This is a PDA derived from [b"vault", mint]. It only signs, never read directly.
    #[account(
        seeds = [b"mint_authority"],
        bump
    )]
    pub reward_authority:UncheckedAccount<'info>,
    pub associated_token_program: Program<'info,AssociatedToken>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info,System>

}