use solana_sdk::{pubkey, pubkey::Pubkey};

pub const KAMINO_RESERVES: [Pubkey; 6] = [
    pubkey!("9DrvZvyWh1HuAoZxvYWMvkf2XCzryCpGgHqrMjyDWpmo"),
    pubkey!("B9spsrMK6pJicYtukaZzDyzsUQLgc3jbx5gHVwdDxb6y"),
    pubkey!("81BgcfZuZf9bESLvw3zDkh7cZmMtDwTPgkCvYu7zx26o"),
    pubkey!("GuWEkEJb5bh8Ai2gaYmZWMTUq8MrFeoaDZ89BrQfB1FZ"),
    pubkey!("Dx8iy2o46sK1DzWbEcznqSKeLbLVeu7otkibA3WohGAj"),
    pubkey!("6QbtpY2jDNcncRFmVf343NThnCdaY8gCAsYATPnYQR9g"),
];

pub const KAMINO_PROGRAM_ID: Pubkey = pubkey!("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD");
pub const KAMINO_SOL_RESERVE: Pubkey = pubkey!("6gTJfuPHEg6uRAijRkMqNc9kan4sVZejKMxmvx2grT1p");
pub const KAMINO_LENDING_JITO_MARKET: Pubkey =
    pubkey!("H6rHXmXoCQvq8Ue81MqNh7ow5ysPa1dSozwW3PU1dDH6");

pub const RESERVE_5: Pubkey = KAMINO_RESERVES[4];

pub const RESERVE_LIQUIDITY: Pubkey = pubkey!("ywaaLvG7t1vXJo8sT3UzE8yzzZtxLM7Fmev64Jbooye");
pub const RESERVE_LIQUIDITY_FEE_RECEIVER: Pubkey =
    pubkey!("EQ7hw63aBS7aPQqXsoxaaBxiwbEzaAiY9Js6tCekkqxf");
pub const SYSVAR_PROGRAM_ID: Pubkey = pubkey!("Sysvar1nstructions1111111111111111111111111");

pub const KAMINO_RATE: u64 = 100_000_u64;
