# Solana Arbitrage Bot

This is an arbitrage trading bot running on Solana blockchain using Jupiter aggregator for token swaps.
To boost bot speed and performance, I have developed this bot using Rust programming language.
The bot monitors large trades in real-time using Helius Laserstream (GRPC) and executes profitable arbitrage opportunities by detecting price differences across different DEX routes.
If you want, I can offer full version and can develop customized advanced project[Advantage: GRPC for tx fetching, direct dex swap, Rust language, multiple relayer services].




## Advanced Version
In arbitrage trading bot, there are two main things: 
one is fetching target token transactions in real-time and other one is executing arbitrage opportunities with minimal latency.
In the basic version, fetching target token transactions is done by RPC websocket (300-500ms) but with the advanced version, I am using Helius Laserstream GRPC for real-time transaction monitoring, and it takes only 50-100ms. Also, the bot uses multiple relayer services (Jito, Nozomi, ZSlot, Astra, Blockrazor, NextBlock) for ultra-fast transaction confirmation to ensure arbitrage opportunities are captured before they disappear. The bot simulates multiple input amounts in parallel to find the most profitable arbitrage path and executes trades using Jupiter API with optimized routes. Of course, it needs more development time because it needs to interact with multiple services and optimize the arbitrage detection algorithm, but otherwise using basic RPC and single route is too easy for development and understanding.




---

## Directory Structure

```
src/
├── config/
│   ├── lazy_setting/
│   │   ├── lazy_arbitrage.rs    # Lazy-loaded arbitrage configurations
│   │   ├── lazy_fee.rs           # Lazy-loaded fee configurations
│   │   ├── lazy_services.rs      # Lazy-loaded service configurations
│   │   ├── lazy_telegram.rs      # Lazy-loaded telegram configurations
│   │   └── mod.rs                # mod file
│   ├── toml_setting/
│   │   ├── toml_arbitrage.rs     # TOML arbitrage settings parser
│   │   ├── toml_credendial.rs    # TOML credential settings parser
│   │   ├── toml_fee.rs           # TOML fee settings parser
│   │   ├── toml_telegram.rs      # TOML telegram settings parser
│   │   └── mod.rs                # mod file
│   └── mod.rs                    # mod file
│
├── constant/
│   ├── alt.rs                    # Address lookup table constants
│   ├── keys.rs                   # Key constants and configurations
│   └── mod.rs                    # mod file
│
├── entry_point/
│   ├── create_nonce.rs           # Nonce account creation utility
│   └── main.rs                   # Main entry point
│
├── modules/
│   ├── encrypt/
│   │   └── mod.rs                # Encryption utilities
│   ├── features/
│   │   ├── process_update.rs     # Process transaction updates and execute arbitrage
│   │   └── mod.rs                # mod file
│   ├── jupiter/
│   │   ├── build_swap_ix.rs      # Build swap instructions
│   │   ├── get_quote.rs          # Get swap quotes from Jupiter
│   │   ├── get_swap_infos.rs     # Get swap information
│   │   ├── get_swap_ix.rs        # Get swap instructions
│   │   ├── ix_data.rs            # Instruction data structures
│   │   └── mod.rs                # mod file
│   ├── lend/
│   │   ├── kamino/
│   │   │   ├── constants.rs      # Kamino lending constants
│   │   │   ├── kamino_account.rs # Kamino account handling
│   │   │   └── mod.rs            # mod file
│   │   └── mod.rs                # mod file
│   ├── stream/
│   │   ├── extract_big_trade.rs  # Extract large trades from transaction updates
│   │   ├── simulate_amount_in.rs # Simulate arbitrage opportunities with different amounts
│   │   └── mod.rs                # mod file
│   ├── telegram/
│   │   └── mod.rs                # Telegram notification service
│   ├── utils/
│   │   ├── alt.rs                # Address lookup table utilities
│   │   ├── block_hash.rs         # Block hash utilities
│   │   ├── nonce.rs              # Nonce account utilities
│   │   └── mod.rs                # mod file
│   └── mod.rs                    # mod file
│
├── lib.rs
└── main.rs
```
---

    


### How To Run
1. Configuration Settings
Edit `Config.toml` file with your settings:
```toml
[credential]
wallet_path = "./id.json"
rpc_endpoint = "https://mainnet.helius-rpc.com?api-key=your_api_key_here"
submit_endpoint = "https://mainnet.helius-rpc.com?api-key=your_api_key_here"
laser_endpoint = "https://laserstream-mainnet-fra.helius-rpc.com"
laser_token = "your_laser_token_here"

[services]
confirm_service = "RPC"        # JITO / LIL_JIT / ZERO_SLOT / NOZOMI / RPC / HELIUS
jito_api_key = "your_jito_api_key"
nozomi_api_key = "your_nozomi_api_key"
zero_slot_key = "your_zero_slot_key"
bloxroute_key = "your_bloxroute_key"
liljit_endpoint = "your_liljit_endpoint"
astralane_key = "your_astralane_key"
blockrazor_key = "your_blockrazor_key"
nextblock_key = "your_nextblock_key"
jupiter_endpoint = "http://localhost:8080"
jupiter_api_key = ""

[arbitrage]
mother_token = [
    { token_addr = "", threshold = 0.000001, min_profit_amount = 0.01, input_amount_range = [0.01, 500], input_amount_steps = 20 },
]
nonce_addr = "your_nonce_account_address"

[fee]
cu = 550000
priority_fee_micro_lamport = 50000
third_party_fee = 0.0001

[telegram]
telegram_bot_key = "your_telegram_bot_key"
group_channel_id = your_group_channel_id

[protect]
protect_key = true
```

2. Create nonce account (if needed):
   Run `make nonce` or `cargo run --bin nonce`

3. Run the bot:
   Run `make main` or `cargo run --bin main`



### Test results
#### Arbitrage Execution
* The bot monitors transactions in real-time using Helius Laserstream GRPC
* Detects large trades on configured mother tokens
* Simulates arbitrage opportunities across multiple input amounts
* Executes profitable trades using Jupiter API with optimized routes
* Uses multiple relayer services for ultra-fast transaction confirmation



### Same Block(Using validator)
- The bot uses nonce accounts for faster transaction processing
- Multiple relayer services (Jito, Nozomi, ZSlot, Astra, Blockrazor) ensure transactions are confirmed in the same block
- Priority fees and compute units are optimized for maximum speed



### Contact Information
- Telegram: https://t.me/DevCutup
- Whatsapp: https://wa.me/13137423660
- Twitter: https://x.com/devcutup
