use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};



use Crate::Escrow;


declare_id!("");



#[program]
pub mod escrow{
    use super::*;

    pub fn take()->Result<()>{

    }


    pub fn make()->Result<()>{
        
    }


    pub fn refund()->Result<()>{
        
    }
}



// #[account]
// #[derive(InitSpace)]
// pub struct Escrow{
//     signer: Pubkey,
//     bump: u8,
//     seeds: u64,
//     mint_account_a : Pubkey,
//     mint_account_b: Pubkey,
//     recieve: u64,
// }





#[derive(Accounts)]
pub struct Make<'info>{
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    mint_account_a : InterfaceAccount<'info,Mint>,
    #[account(
        mint::token_program = token_program
    )]
    mint_account_b : InterfaceAccount<'info,Mint>,
    #[account(
        mut,
        associated_token::mint = mint_account_a,
        associated_token::authority = signer,
        associated_token::token_program = token_program 
    )]
    signer_ata_a: InterfaceAccount<'info,TokenAccount>,
    #[account(
        init,
        payer=signer,
        space = 8+ Escrow::InitSpace,
        seeds = [b"Escrow",signer.key().as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info,Escrow>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = mint_account_a,
        associated_token::token_program = token_program,
        associated_token::authority = escrow,
    )]
    pub vault : InterfaceAccount<'info,TokenAccount>,
    associated_token_program:Program<'info,AssociatedToken>,
    token_program: Program<'info,TokenInterface>,
    system_program: Program<'info,System>
}




impl<'info> Make<'info>{
    pub fn save_escrow(&self, seed:u64,recieve:u64,bumps:&MakeBumps){
        self.escrow.set_inner(
            Escrow{
                seed,
                recieve,
                bump: bumps.escrow,
                mint_account_a: self.mint_account_a,
                mint_account_b: self.mint_account_b  
            }
        );
    }



    pub fn deposit(&self,amount:u64){
        let transfer_accounts = TransferChecked{
            from: self.signer_ata_a.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.mint_account_a.to_account_info(),
            authority: self.signer.to_account_info()
        };


        let cpi = CpiContext::new(self.token_program.to_account_info(),transfer_accounts);


        transfer_checked(cpi, amount, self.mint_account_a.decimals);
        
    }
}







#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    taker: Signer<'info>,
    #[account(mut)]
    signer: Signer<'info>,
    pub mint_account_a : InterfaceAccount<'info,Mint>,
    pub mint_account_b : InterfaceAccount<'info,Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_account_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: InterfaceAccount<'info,TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_account_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut
        associated_token::mint = mint_account_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info,TokenAccount>,
    #[account(
        has_one = mint_account_a,
        mut,
        close = signer,
        has_one = mint_account_b,
        has_one = signer,
        seeds = [b"Escrow",signer.key.as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow : Account<'info,Escrow>,
    #[account(
        
    )]
    
}