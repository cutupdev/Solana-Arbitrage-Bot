use cab_jup_enhanced_bot::{PRIVATE_KEY, RPC_CLIENT};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    nonce::State, pubkey::Pubkey, signature::{Keypair, Signer}, system_instruction, transaction::Transaction
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let nonce_account = Keypair::new();

    let nonce_rent = RPC_CLIENT.get_minimum_balance_for_rent_exemption(State::size()).await?;
    let instr = system_instruction::create_nonce_account(
        &PRIVATE_KEY.pubkey(),
        &nonce_account.pubkey(),
        &PRIVATE_KEY.pubkey(), // Make the fee payer the nonce account authority
        nonce_rent,
    );

    let mut tx = Transaction::new_with_payer(&instr, Some(&PRIVATE_KEY.pubkey()));

    let blockhash = RPC_CLIENT.get_latest_blockhash().await?;
    tx.try_sign(&[&nonce_account, &PRIVATE_KEY], blockhash)?;


    RPC_CLIENT.send_and_confirm_transaction(&tx).await?;

    println!("Nonce account created: {}", nonce_account.pubkey());
    Ok(())
}
