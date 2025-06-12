use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked,Mint,TokenAccount,TokenInterface,TransferChecked}
};

use crate::Escrow;



#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Deposit<'info>{
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_account_a: Account<'info,Mint>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_account_b: Account<'info,Mint>,
    #[account(
        mut,
        associated_token::mint = mint_account_a,
        associated_token::authority =signer,
        associated_token::token_program = token_program
    )]
    pub associated_token_account_a : InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_account_a,
        associated_token::authority =signer,
        associated_token::token_program = token_program
    )]
    pub escrow: Program<'info,Escrow>,
    #[account(
        init,
        payer=signer,
        associated_token::mint = mint_account_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info,TokenAccount>,
    pub associated_token_program: Program<'info,AssociatedToken>,
    pub system_program: Program<'info,System>,
    pub token_program:Program<'info,TokenInterface>
}




impl<'info> Deposit<'info>{

    pub fn deposit(&mut self,seed:u64,recieve:u64,bumps:&MakeBumps)

}