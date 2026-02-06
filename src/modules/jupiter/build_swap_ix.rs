use crate::{
    JUPITER_EVENT_AUTH, JUPITER_PROGRAM_ADDR, PUBKEY, ROUTE_DISCRIMINATOR, RouteArgs,
    TOKEN_PROGRAM_ID,
};
use borsh::to_vec;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use spl_associated_token_account::get_associated_token_address;

/// Builds a Jupiter swap instruction from route arguments and account metadata.
pub fn build_swap_ix(
    route_args: RouteArgs,
    remaining_accounts: Vec<AccountMeta>,
    mother_token: Pubkey,
) -> Instruction {
    // Serializes route args with discriminator and constructs instruction with required accounts
    todo!("Implementation removed for NDA compliance")
}
