//! CLI definition and entrypoint

use std::fmt::Debug;
use clap::{Parser, arg, Args};
use primitive_types::*;

/// Print the hash based on user arg.
fn print_primitive(hash: impl Debug, format: FormatArgs) {
    let show_hex = format.hex;

    match show_hex {
        true => println!("{hash:?}"),
        false => {
            let string = format!("{hash:?}");
            let sub = &string[2..];
            println!("{sub}");
        },
    }
}

/// Parse CLI command and generate random fixed-hash.
pub fn run() -> eyre::Result<()> {
    let opt = Cli::parse();

    println!("opt: {opt:?}");

    match opt {
        Cli::H128(format) => {
            let hash = H128::random();
            print_primitive(hash, format)
        }
        Cli::H160(format) => {
            let hash = H160::random();
            print_primitive(hash, format)
        }
        Cli::H256(format) => {
            let hash = H256::random();
            print_primitive(hash, format)
        }
        Cli::H384(format) => {
            let hash = H384::random();
            print_primitive(hash, format)
        }
        Cli::H512(format) => {
            let hash = H512::random();
            print_primitive(hash, format)
        }
        Cli::H768(format) => {
            let hash = H768::random();
            print_primitive(hash, format)
        }
        Cli::U128(format) => {
            let random: u128 = rand::random();
            let hash = U128::from(random);
            print_primitive(hash, format)
        }
        Cli::U256(format) => {
            let random: u128 = rand::random();
            let hash = U256::from(random);
            print_primitive(hash, format)
        }
        Cli::U512(format) => {
            let random: u128 = rand::random();
            let hash = U512::from(random);
            print_primitive(hash, format)
        }
    };

    Ok(())
}

#[derive(Parser, Debug)]
enum Cli {
    /// H128:
    /// Fixed-size uninterpreted hash type with 16 bytes (128 bits) size.
    H128(FormatArgs),
    /// H160:
    /// Fixed-size uninterpreted hash type with 20 bytes (160 bits) size.
    H160(FormatArgs),
    /// H256:
    /// Fixed-size uninterpreted hash type with 32 bytes (256 bits) size.
    H256(FormatArgs),
    /// H384:
    /// Fixed-size uninterpreted hash type with 48 bytes (384 bits) size.
    H384(FormatArgs),
    /// H512:
    /// Fixed-size uninterpreted hash type with 64 bytes (512 bits) size.
    H512(FormatArgs),
    /// H768:
    /// Fixed-size uninterpreted hash type with 96 bytes (768 bits) size.
    H768(FormatArgs),
    /// U128:
    /// Little-endian large integer type 128-bit unsigned integer.
    U128(FormatArgs),
    /// U256:
    /// Little-endian large integer type 256-bit unsigned integer.
    U256(FormatArgs),
    /// U512:
    /// Little-endian large integer type 512-bits unsigned integer.
    U512(FormatArgs),
}

#[derive(Args, Debug)]
struct FormatArgs {
    #[arg(short='x', long, help = "print the leading 0x for hexidecimal format")]
    hex: bool,
}
