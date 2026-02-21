//! Solana program addresses, token information, and related constants
//!
//! This module contains all program addresses, token mint addresses,
//! and mapping data used for identifying DEX programs and tokens.

use phf::phf_map;
use solana_sdk::pubkey::Pubkey;

/// Mapping of Solana program addresses to their DEX/protocol names
/// Used for identifying which DEX a transaction interacts with
pub const PROGRAM_MAP: phf::Map<&'static str, &'static str> = phf_map! {
  "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB" => "Meteora",
  "5U3EU2ubXtK84QcRjWVmYt9RaDyA8gKxdUrPFXmZyaki" => "Virtuals",
  "HEAVENoP2qxoeuF8Dj2oT1GHEnu49U5mJYkdeC8BAX2o" => "Heaven",
  "ALPHAQmeA7bjrVuccPsYPiCvsi428SNwte66Srvs4pHA" => "AlphaQ",
  "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo" => "Meteora DLMM",
  "5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx" => "Sanctum Infinity",
  "DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1" => "Orca V1",
  "9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp" => "HumidiFi",
  "NUMERUNsFCP3kuNmWZuXtm1AaQCPj9uw6Guv2Ekoi5P" => "Perena",
  "opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb" => "OpenBook V2",
  "Gswppe6ERWKpUTXvRPfXdzHhiCyJvLadVvXGfdpBqcE1" => "Guacswap",
  "SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ" => "Saber",
  "SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe" => "SolFi",
  "TessVdML9pBGgG9yGks7o4HewRaXVAMuoVj4x83GLQH" => "TesseraV",
  "endoLNCKTqDn8gSVnN2hDdpgACUPWHZTwoYnnMybpAT" => "Solayer",
  "LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj" => "Raydium Launchlab",
  "AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6" => "Aldrin",
  "CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4" => "Aldrin V2",
  "WooFif76YGRNjk1pA8wCsN67aQsD9f9iLsz4NcJ1AVb" => "Woofi",
  "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP" => "Orca V2",
  "DecZY86MU5Gj7kppfUCEmd4LbXXuyZH1yHaP2NTqdiZB" => "Saber (Decimals)",
  "SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF" => "SolFi V2",
  "HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt" => "Invariant",
  "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY" => "Phoenix",
  "GAMMA7meSFWaBXF25oSUgmGRwaW6sCMFLmBNiMSdbHVT" => "GooseFX GAMMA",
  "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P" => "Pump.fun",
  "boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4" => "Boop.fun",
  "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA" => "Pump.fun Amm",
  "goonERTdGsjnkZqWuVjs73BZ3Pb9qoCUdBUL17BnS5j" => "GoonFi",
  "DSwpgjMvXhtGn6BsbqmacdBZyfLj6jSWf3HJpdJtmg6N" => "DexLab",
  "swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ" => "Stabble Stable Swap",
  "Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j" => "StepN",
  "cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG" => "Meteora DAMM v2",
  "HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq" => "PancakeSwap",
  "stkitrT1Uoy18Dk1fTrgPw8W6MVzoCfYoAFT4MLsmhq" => "Sanctum",
  "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C" => "Raydium CP",
  "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" => "Raydium",
  "CLMM9tUoggJu2wagPkkqs9eFG4BWhVBZWkP1qv3Sp7tR" => "Crema",
  "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK" => "Raydium CLMM",
  "MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky" => "Mercurial",
  "PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu" => "Perps",
  "H8W3ctz92svYg6mkn1UtGfu2aQr2fnUFHM1RhScEtQDt" => "Cropper",
  "DEXYosS6oEGvk8uCDayvwEZz4qEyDJRf9nFgYCaqPMTm" => "1DEX",
  "fUSioN9YKKSa3CUC2YUc4tPkHJ5Y6XW1yz8y6F7qWz9" => "DefiTuna",
  "SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr" => "Saros",
  "AQU1FRd7papthgdrwPTTq5JacJh8YtwEXaBfKU3bTz45" => "Aquifer",
  "SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8" => "Token Swap",
  "PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP" => "Penguin",
  "treaf4wWBBty3fHdyBpo35Mz84M8k3heKXmjmi9vFt5" => "Helium Network",
  "REALQqNEomY6cQGZJUGwywTBD2UmDT32rZcNnfxQ5N2" => "Byreal",
  "BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p" => "Bonkswap",
  "ZERor4xhbUycZ6gb9ntrhqscUcZmAbQDjEAtCf4hbZY" => "ZeroFi",
  "FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X" => "FluxBeam",
  "srAMMzfVHVAtgSJc8iH6CfKzuWuUTzLHVCE81QU1rgi" => "Gavel",
  "2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c" => "Lifinity V2",
  "swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW" => "Stabble Weighted Swap",
  "obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y" => "Obric V2",
  "dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN" => "Dynamic Bonding Curve",
  "MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG" => "Moonit",
  "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" => "Whirlpool"
};

/// Route discriminator bytes for Jupiter swap instructions
pub const ROUTE_DISCRIMINATOR: [u8; 8] = [229, 23, 203, 151, 122, 227, 173, 42];

// ============================================================================
// Token Addresses
// ============================================================================

/// Wrapped SOL (WSOL) token mint address
pub const WSOL: Pubkey = Pubkey::from_str_const("So11111111111111111111111111111111111111112");

// ============================================================================
// Program Addresses
// ============================================================================

/// Solana Token Program ID
pub const TOKEN_PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

/// Jupiter Aggregator Program Address
pub const JUPITER_PROGRAM_ADDR: Pubkey =
    Pubkey::from_str_const("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

/// Jupiter Transfer Authority
pub const JUPITER_TRANSFER_AUTH: Pubkey =
    Pubkey::from_str_const("9nnLbotNTcUhvbrsA6Mdkx45Sm82G35zo28AqUvjExn8");

/// Jupiter Event Authority
pub const JUPITER_EVENT_AUTH: Pubkey =
    Pubkey::from_str_const("D8cy77BBepLMngZx6ZukaTff5hCt1HrWyKk3Hnd9oitf");

// ============================================================================
// Token Information
// ============================================================================

/// Token information structure containing mint address, symbol, and decimals
#[derive(Debug, Clone)]
pub struct TokenInfo {
    /// Token mint address
    pub mint: &'static str,
    /// Token symbol (e.g., "USDC", "SOL")
    pub symbol: &'static str,
    /// Number of decimal places for the token
    pub decimals: u8,
}

/// List of popular tokens with their metadata
/// Used for quick token lookups and conversions
pub const POPULAR_TOKEN_INFO: &[TokenInfo] = &[
    TokenInfo {
        mint: "So11111111111111111111111111111111111111112",
        symbol: "WSOL",
        decimals: 9,
    },
    TokenInfo {
        mint: "9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump",
        symbol: "Fartcoin",
        decimals: 6,
    },
    TokenInfo {
        mint: "KMNo3nJsBXfcpJTVhZcXLW7RmTwTt4GVFE7suUBo9sS",
        symbol: "KMNO",
        decimals: 6,
    },
    TokenInfo {
        mint: "27G8MtK7VtTcCHkpASjSDdkWWYfoqT6ggEuKidVJidD4",
        symbol: "JLP",
        decimals: 6,
    },
    TokenInfo {
        mint: "2zMMhcVQEXDtdE6vsFS7S7D5oUodfJHE8vd1gnBouauv",
        symbol: "PENGU",
        decimals: 6,
    },
    TokenInfo {
        mint: "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh",
        symbol: "WBTC",
        decimals: 8,
    },
    TokenInfo {
        mint: "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R",
        symbol: "RAY",
        decimals: 6,
    },
    TokenInfo {
        mint: "6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN",
        symbol: "TRUMP",
        decimals: 6,
    },
    TokenInfo {
        mint: "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj",
        symbol: "stSOL",
        decimals: 9,
    },
    TokenInfo {
        mint: "7kbnvuGBxxj8AG9qp8Scn56muWGaRaFqxg1FsRp3PaFT",
        symbol: "UXD",
        decimals: 6,
    },
    TokenInfo {
        mint: "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs",
        symbol: "ETH",
        decimals: 8,
    },
    TokenInfo {
        mint: "85VBFQZC9TZkfaptBWjvUw7YbZjy52A6mjtPGjstQAmQ",
        symbol: "W",
        decimals: 6,
    },
    TokenInfo {
        mint: "9vMJfxuKxXBoEa7rM12mYLMwTacLMLDJqHozw96WQL8i",
        symbol: "UST",
        decimals: 6,
    },
    TokenInfo {
        mint: "ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx",
        symbol: "ATLAS",
        decimals: 8,
    },
    TokenInfo {
        mint: "cbbtcf3aa214zXHbiAZQwf4122FBYbraNdFqgw4iMij",
        symbol: "cbBTC",
        decimals: 8,
    },
    TokenInfo {
        mint: "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263",
        symbol: "Bonk",
        decimals: 5,
    },
    TokenInfo {
        mint: "Ea5SjE2Y6yvCeW5dYTn7PYMuW5ikXkvbGdcmSnXeaLjS",
        symbol: "PAI",
        decimals: 6,
    },
    TokenInfo {
        mint: "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm",
        symbol: "$WIF",
        decimals: 6,
    },
    TokenInfo {
        mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        symbol: "USDC",
        decimals: 6,
    },
    TokenInfo {
        mint: "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
        symbol: "USDT",
        decimals: 6,
    },
    TokenInfo {
        mint: "HgH6C35Ncz6SfhU8L4zeWZ18tHu6BBpZ7fU37KgxYoG3",
        symbol: "CYCLIQ",
        decimals: 6,
    },
    TokenInfo {
        mint: "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn",
        symbol: "JitoSOL",
        decimals: 9,
    },
    TokenInfo {
        mint: "J6pQQ3FAcJQeWPPGppWRb4nM8jU3wLyYbRrLh7feMfvd",
        symbol: "2Z",
        decimals: 8,
    },
    TokenInfo {
        mint: "jupSoLaHXQiZZTSfEWMTRRgpnyFm8f6sZdosWBjx93v",
        symbol: "JupSOL",
        decimals: 9,
    },
    TokenInfo {
        mint: "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
        symbol: "JUP",
        decimals: 6,
    },
    TokenInfo {
        mint: "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So",
        symbol: "mSOL",
        decimals: 9,
    },
    TokenInfo {
        mint: "USD1ttGY1N17NEEHLmELoaybftRBUSErhqYiQzvEmuB",
        symbol: "USD1",
        decimals: 6,
    },
    TokenInfo {
        mint: "USDH1SM1ojwWUga67PGrgFWUHibbjqMvuMaDkRJTgkX",
        symbol: "USDH",
        decimals: 6,
    },
];

/// List of intermediate tokens used for arbitrage routing
/// These tokens are commonly used as intermediate steps in multi-hop swaps
pub const INTERMEDIATE_TOKEN: &[&'static str] = &[
