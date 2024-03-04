#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
//#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;
use c_kzg::{KzgSettings, BYTES_PER_G1_POINT, BYTES_PER_G2_POINT};

risc0_zkvm::guest::entry!(main);

mod trusted_setup;
use trusted_setup::{g1_bytes, g2_bytes};

use std::mem;

fn main() {
    // TODO: Implement your guest code here

    // load the setup
    let g1_transmuted: &[[u8; BYTES_PER_G1_POINT]; 4096] = unsafe { mem::transmute(g1_bytes) };
    let g2_transmuted: &[[u8; BYTES_PER_G2_POINT]; 65] = unsafe { mem::transmute(g2_bytes) };
    let settings = KzgSettings::load_trusted_setup(g1_transmuted, g2_transmuted)
        .unwrap();
    // read the input
    let input: u32 = env::read();

    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}
