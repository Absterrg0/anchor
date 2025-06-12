use std::thread::current;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TransferChecked,transfer_checked};
use anchor_lang::system_program::*;


declare_id!("");

#[error_code]
pub enum StakeError {
    #[msg("Amount must be greater than 0")]
    InvalidAmount,
    #[msg("Insufficient staked amount")]
    InsufficientStake,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Arithmetic underflow")]
    Underflow,
    #[msg("Invalid timestamp")]
    InvalidTimestamp,
}

const POINTS_PER_SOL_PER_DAY: u64 = 1_000_000; // Using micro-points for precision
const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
const SECONDS_PER_DAY: u64 = 86_400;


const POINTS_PER_LAMPORT_PER_SECOND: u128 = 
    (POINTS_PER_SOL_PER_DAY as u128)
    / (LAMPORTS_PER_SOL as u128)
    / (SECONDS_PER_DAY as u128);

#[program]
pub mod staking{


    pub fn initialise_pda()->Result<()>{

    }

    pub fn stake()->Result<()>{

    }

    pub fn unstake()->Result<()>{
        
    }

    pub fn claim_points()->Result<()>{
        
    }
}



pub fn update_points(staking_pda_account: &mut StakeAccount, time: i64)->Result<()>{


    let time_elapsed = time.checked_sub(staking_pda_account.last_updated_stake_time).ok_or(StakeError::InvalidTimestamp)?;

    if time_elapsed>0 && staking_pda_account.stake_amount>0{
        let new_points = calculate_points_earned(staking_pda_account.stake_amount, time_elapsed)?;
        staking_pda_account.total_points = staking_pda_account.total_points.checked_add(new_points).ok_or(StakeError::Overflow)?;
    }



    Ok(())
}



pub fn calculate_points_earned(staked_amount: u64, time_elapsed: i64 )->Result<u64>{


    let points = (staked_amount as u128).checked_mul(time_elapsed as u128).ok_or(StakeError::Overflow)?.checked_mul(POINTS_PER_LAMPORT_PER_SECOND).ok_or(StakeError::Overflow)?;

    

    Ok(points as u64)
}


#[account]
pub struct StakeAccount{
    pub owner: Pubkey,
    pub stake_amount: u64,
    pub last_updated_stake_time : i64,
    pub bump: u8,
    pub total_points: u64
}




#[derive(Accounts)]
pub struct initialise_pda<'info>{
    #[account(mut)]
    owner: Signer<'info>,
    pub system_program : Program<'info,System>,
    #{account(
        init,
        payer=owner,
        space = 8+32+8+8+1+8,
        bump,
        seeds = [b"staking",owner.key.as_ref()]
    )}
    pub staking_pda_account: Account<'info,StakeAccount>
}



impl<'info> initialise_pda<'info>{

    pub fn create_pda(&self,init_amount:u64)->Result<()>{
        let pda_account = self.staking_pda_account;
        let clock = Clock::get()?;
        pda_account.owner = self.owner.key;
        if init_amount>0{
            pda_account.stake_amount = init_amount;
        }
        else{
            pda_account.stake_amount=0;
        }

        pda_account.last_updated_stake_time = clock.unix_timestamp;
        pda_account.bump = self.bumps.staking_pda_account;
        
        Ok(())
    }
}



#[derive(Accounts)]
pub struct stake<'info>{
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"staking",owner.key.as_ref()],
        bump = staking_pda_account.bump
    )]
    pub staking_pda_account: Account<'info,StakeAccount>,
    pub system_program: Program<'info,System>
}


impl<'info> stake<'info>{

    pub fn stake(&mut self, amount:u64)-> Result<()>{

            let pda_account = &mut self.staking_pda_account;
            let clock = Clock::get()?;
            update_points(pda_account, clock.unix_timestamp)?;

            let transfer_accounts = Transfer{
                from: self.owner.to_account_info(),
                to: pda_account.to_account_info(),
            };


            let cpi_context = CpiContext::new(self.system_program.to_account_info(),transfer_accounts);


            transfer(cpi_context, amount);

            pda_account.stake_amount = pda_account.stake_amount.checked_add(amount).ok_or(StakeError::Overflow)?;

        Ok(())
    }
}





#[derive(Accounts)]
pub struct unstake<'info>{
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"staking",owner.key.as_ref()],
        bump = staking_pda_account.bump
    )]
    pub staking_pda_account: Account<'info,StakeAccount>,
    pub system_program: Program<'info,System>
}



impl<'info> unstake<'info>{

    pub fn unstake(&mut self, amount:u64)->Result<()>{

        let staking_account = &mut self.staking_pda_account;
        let clock = Clock::get()?;

        update_points( staking_account, clock.unix_timestamp);

        let owner_key = self.owner.key();
        let seeds = &[
            b"staking",
            owner_key.as_ref(),
            &[staking_account.bump]
        ];
        let signer = &[&seeds[..]];

        let transfer_accounts = Transfer{
            from:staking_account.to_account_info(),
            to: self.owner.to_account_info(),

        };

        let cpi_context = CpiContext::new_with_signer(self.system_program.to_account_info(), transfer_accounts, signer);


        transfer(cpi_context,amount);

        staking_account.stake_amount = staking_account.stake_amount.checked_sub(amount).ok_or(StakeError::Underflow)?;

        Ok(())
    }   
}
