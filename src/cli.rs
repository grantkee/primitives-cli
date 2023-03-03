//! CLI definition and entrypoint

use clap::Parser;
use primitive_types::*;

/// Parse CLI command and generate random fixed-hash.
pub fn run() -> eyre::Result<()> {
    let opt = Cli::parse();

    match opt {
        Cli::H128 => {
            let hash = H128::random();
            println!("{opt:?}:\n{hash:?}");
        }
        Cli::H160 => {
            let hash = H160::random();
            println!("{opt:?}:\n{hash:?}");
        }
        Cli::H256 => {
            let hash = H256::random();
            println!("{opt:?}:\n{hash:?}");
        }
        Cli::H384 => {
            let hash = H384::random();
            println!("{opt:?}:\n{hash:?}");
        }
        Cli::H512 => {
            let hash = H512::random();
            println!("{opt:?}:\n{hash:?}");
        }
        Cli::H768 => {
            let hash = H768::random();
            println!("{opt:?}:\n{hash:?}");
        }
        Cli::U128 => {
            let random: u128 = rand::random();
            let hash = U128::from(random);
            println!("{opt:?}:\n{hash:?}");
        }
        Cli::U256 => {
            let random: u128 = rand::random();
            let hash = U256::from(random);
            // let num: U256 = U256::zero()
                // .saturating_add(U128::MAX.into())
                // .saturating_add(U128::MAX.into());
            println!("{opt:?}:\n{hash:?}");
        }
        Cli::U512 => {
            let random: u128 = rand::random();
            let hash = U512::from(random);
            println!("{opt:?}:\n{hash:?}");
        }
    };

    Ok(())
}

#[derive(Parser, Debug)]
enum Cli {
    /// H128	Fixed-size uninterpreted hash type with 16 bytes (128 bits) size.
    // #[command(name = "h128")]
    H128,
    /// H160	Fixed-size uninterpreted hash type with 20 bytes (160 bits) size.
    H160,
    /// H256	Fixed-size uninterpreted hash type with 32 bytes (256 bits) size.
    H256,
    /// H384	Fixed-size uninterpreted hash type with 48 bytes (384 bits) size.
    H384,
    /// H512	Fixed-size uninterpreted hash type with 64 bytes (512 bits) size.
    H512,
    /// H768	Fixed-size uninterpreted hash type with 96 bytes (768 bits) size.
    H768,
    /// U128	Little-endian large integer type 128-bit unsigned integer.
    U128,
    /// U256	Little-endian large integer type 256-bit unsigned integer.
    U256,
    /// U512	Little-endian large integer type 512-bits unsigned integer.
    U512,
}