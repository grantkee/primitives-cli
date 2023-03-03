//! CLI definition and entrypoint

use clap::{Parser, arg, Args};
use primitive_types::*;

/// Parse CLI command and generate random fixed-hash.
pub fn run() -> eyre::Result<()> {
    let opt = Cli::parse();

    println!("opt: {opt:?}");

    let (hash, hex) = match opt {
        Cli::H128(hex) => {
            let hash = H128::random();
            (hash.to_string(), hex)
        }
        Cli::H160(hex) => {
            let hash = H160::random();
            println!("hex: {hex:?}");
            (hash.to_string(), hex)
        }
        Cli::H256(hex) => {
            let hash = H256::random();
            (hash.to_string(), hex)
        }
        Cli::H384(hex) => {
            let hash = H384::random();
            (hash.to_string(), hex)
        }
        Cli::H512(hex) => {
            let hash = H512::random();
            (hash.to_string(), hex)
        }
        Cli::H768(hex) => {
            let hash = H768::random();
            (hash.to_string(), hex)
        }
        Cli::U128(hex) => {
            let random: u128 = rand::random();
            let hash = U128::from(random);
            (hash.to_string(), hex)
        }
        Cli::U256(hex) => {
            let random: u128 = rand::random();
            let hash = U256::from(random);
            // let num: U256 = U256::zero()
                // .saturating_add(U128::MAX.into())
                // .saturating_add(U128::MAX.into());
            (hash.to_string(), hex)
        }
        Cli::U512(hex) => {
            let random: u128 = rand::random();
            let hash = U512::from(random);
            (hash.to_string(), hex)
        }
    };

    match hex.format {
        true => println!("{hash}"),
        false => println!("{hash}"),
    }

    Ok(())
}

#[derive(Parser, Debug)]
enum Cli {
    /// H128	Fixed-size uninterpreted hash type with 16 bytes (128 bits) size.
    H128(FormatArgs),
    /// H160	Fixed-size uninterpreted hash type with 20 bytes (160 bits) size.
    H160(FormatArgs),
    /// H256	Fixed-size uninterpreted hash type with 32 bytes (256 bits) size.
    H256(FormatArgs),
    /// H384	Fixed-size uninterpreted hash type with 48 bytes (384 bits) size.
    H384(FormatArgs),
    /// H512	Fixed-size uninterpreted hash type with 64 bytes (512 bits) size.
    H512(FormatArgs),
    /// H768	Fixed-size uninterpreted hash type with 96 bytes (768 bits) size.
    H768(FormatArgs),
    /// U128	Little-endian large integer type 128-bit unsigned integer.
    U128(FormatArgs),
    /// U256	Little-endian large integer type 256-bit unsigned integer.
    U256(FormatArgs),
    /// U512	Little-endian large integer type 512-bits unsigned integer.
    U512(FormatArgs),
}

#[derive(Args, Debug)]
struct FormatArgs {
    #[arg(short, long)]
    format: bool,
}
