use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use spl_associated_token_account::get_associated_token_address;

use crate::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct KaminoLendAccount {
    pub user_transfer_authority: Pubkey,
    pub lending_market_authority: Pubkey,
    pub lending_market: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_liquidity: Pubkey,
    pub user_liquidity: Pubkey,
    pub reserve_liquidity_fee_receiver: Pubkey,
    pub referrer_token_state: Pubkey,
    pub referrer_account: Pubkey,
    pub sysvar_instruction_id: Pubkey,
    pub token_program: Pubkey,
}

impl KaminoLendAccount {
    pub fn new() -> Self {
        let user_liquidity = get_associated_token_address(&PUBKEY, &WSOL);

        Self {
            user_transfer_authority: *PUBKEY,
            lending_market_authority: RESERVE_5,
            lending_market: KAMINO_LENDING_JITO_MARKET,
            reserve: KAMINO_SOL_RESERVE,
            reserve_liquidity_mint: WSOL,
            reserve_liquidity: RESERVE_LIQUIDITY,
            user_liquidity: user_liquidity,
            reserve_liquidity_fee_receiver: RESERVE_LIQUIDITY_FEE_RECEIVER,
            referrer_token_state: KAMINO_PROGRAM_ID,
            referrer_account: KAMINO_PROGRAM_ID,
            sysvar_instruction_id: SYSVAR_PROGRAM_ID,
            token_program: TOKEN_PROGRAM_ID,
        }
    }

    pub fn create_flash_borrow_instruction(&self, amount: u64) -> Instruction {
        let mut data: Vec<u8> = Vec::new();

        data.extend([135, 231, 52, 167, 7, 52, 212, 193]);
        data.extend(amount.to_le_bytes());

        let keys = vec![
            AccountMeta::new(self.user_transfer_authority, true),
            AccountMeta::new_readonly(self.lending_market_authority, false),
            AccountMeta::new_readonly(self.lending_market, false),
            AccountMeta::new(self.reserve, false),
            AccountMeta::new_readonly(self.reserve_liquidity_mint, false),
            AccountMeta::new(self.reserve_liquidity, false),
            AccountMeta::new(self.user_liquidity, false),
            AccountMeta::new(self.reserve_liquidity_fee_receiver, false),
            AccountMeta::new(self.referrer_token_state, false),
            AccountMeta::new(self.referrer_account, false),
            AccountMeta::new_readonly(self.sysvar_instruction_id, false),
            AccountMeta::new_readonly(self.token_program, false),
        ];

        Instruction {
            program_id: self.referrer_account,
            accounts: keys,
            data,
        }
    }

    pub fn create_flash_repay_instruction(&self, amount: u64, lend_ix_idx: u8) -> Instruction {
        let mut data: Vec<u8> = Vec::new();

        data.extend([185, 117, 0, 203, 96, 245, 180, 186]);
        data.extend(amount.to_le_bytes());
        data.push(lend_ix_idx);

        let keys = vec![
            AccountMeta::new(self.user_transfer_authority, true),
            AccountMeta::new_readonly(self.lending_market_authority, false),
            AccountMeta::new_readonly(self.lending_market, false),
            AccountMeta::new(self.reserve, false),
            AccountMeta::new_readonly(self.reserve_liquidity_mint, false),
            AccountMeta::new(self.reserve_liquidity, false),
            AccountMeta::new(self.user_liquidity, false),
            AccountMeta::new(self.reserve_liquidity_fee_receiver, false),
            AccountMeta::new(self.referrer_token_state, false),
            AccountMeta::new(self.referrer_account, false),
            AccountMeta::new_readonly(self.sysvar_instruction_id, false),
            AccountMeta::new_readonly(self.token_program, false),
        ];

        Instruction {
            program_id: self.referrer_account,
            accounts: keys,
            data,
        }
    }
}
