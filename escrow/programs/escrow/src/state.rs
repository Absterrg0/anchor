use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Escrow{
    pub bump: u32,
    pub seed: u64,
    pub mint_account_a: Pubkey,
    pub mint_account_b: Pubkey,
    pub recieve : u64,
    pub maker: Pubkey
}