# Solana-Arbitrage-Bot
This is an arbitrage bot that gets profit by token price difference between several dexs. It finds prifitable router and confirm transaction. This is not full source code, just sharing old version for giving vision about solana arbitrage bot, feel free to reach out of me when you need full code or custom arbitrage bot [Whatsapp: https://wa.me/13137423660 Or Telegram: https://t.me/DevCutup].


## Main featurs 
### Dex supports 
- Pumpfun
- Pumpswap
- Raydium
- Meteora

### Workflow
#### Off-chain
On off-chain side, it fetches token's price on several dexs, and determine profitable router.
#### On-chain
Once bot finds profitable router on off-chain side, it builds transaction then send to on-chain program. On-chain program confirm it by CPIs to multi dexs.


## How to install
1. Clone the repository
```bash
git clone https://github.com/cutupdev/Solana-Arbitrage-Bot.git
cd Solana-Arbitrage-Bot
```
2.  Off-chain
- Build the project
```bash
cargo build
```
- Run the project
```bash
cargo run
```
3. On-chain
- Build smart contract
```bash
anchor build
```
- Deploy smart contract
```bash
anchor deploy
```


## Folders description
- `offchain/`: off-chain arbitrage bot code 
- `swap/`: on-chain swap program
- `pools/`: dex pool metadata
- `onchain/`: analysis of other arbitrage swaps
- `mainnet/`: fork mainnet account states to test swap input/output estimates


### Contact Information
- Telegram: https://t.me/DevCutup
- Whatsapp: https://wa.me/13137423660
- Twitter: https://x.com/devcutup
