//! Address Lookup Table (ALT) constants
//!
//! This module contains constants related to Address Lookup Tables (ALTs)
//! used for optimizing transaction size on Solana.

use solana_sdk::pubkey::Pubkey;

/// External Address Lookup Table addresses
/// ALTs allow transactions to reference more accounts while staying within size limits
pub const ALT_EXTERNAL: [Pubkey; 1] = [Pubkey::from_str_const(
    "3pqmFC8JcBNoZQqojvaUqTi7ydxa3EdVvbFGb7PZMqMY",
)];

// Additional ALT addresses (commented for reference):
//   @3pqmFC8JcBNoZQqojvaUqTi7ydxa3EdVvbFGb7PZMqMY
//   So11111111111111111111111111111111111111112
//   EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
//   Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB
//   9DrvZvyWh1HuAoZxvYWMvkf2XCzryCpGgHqrMjyDWpmo
//   B9spsrMK6pJicYtukaZzDyzsUQLgc3jbx5gHVwdDxb6y
//   81BgcfZuZf9bESLvw3zDkh7cZmMtDwTPgkCvYu7zx26o
//   GuWEkEJb5bh8Ai2gaYmZWMTUq8MrFeoaDZ89BrQfB1FZ
//   Dx8iy2o46sK1DzWbEcznqSKeLbLVeu7otkibA3WohGAj
//   6QbtpY2jDNcncRFmVf343NThnCdaY8gCAsYATPnYQR9g
//   KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD
//   6gTJfuPHEg6uRAijRkMqNc9kan4sVZejKMxmvx2grT1p
//   H6rHXmXoCQvq8Ue81MqNh7ow5ysPa1dSozwW3PU1dDH6
//   ywaaLvG7t1vXJo8sT3UzE8yzzZtxLM7Fmev64Jbooye
//   EQ7hw63aBS7aPQqXsoxaaBxiwbEzaAiY9Js6tCekkqxf
