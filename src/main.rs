#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;
use c_kzg::{Blob, BYTES_PER_G1_POINT, BYTES_PER_G2_POINT, FIELD_ELEMENTS_PER_BLOB, KzgSettings};
mod trusted_setup;
use trusted_setup::{g1_bytes, g2_bytes};
const NUM_G2_POINTS: usize = 65;
//use kzg::setup::Setup;

risc0_zkvm::guest::entry!(main);


fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    // TODO: do something with the input

    // write public output to the journal

    let g1_ref: &[[u8; BYTES_PER_G1_POINT]; FIELD_ELEMENTS_PER_BLOB] = unsafe {
        let ptr = g1_bytes.as_ptr() as *const [[u8; BYTES_PER_G1_POINT]; FIELD_ELEMENTS_PER_BLOB];
        &*ptr
    };
    let g2_ref: &[[u8; BYTES_PER_G2_POINT]; NUM_G2_POINTS] = unsafe {
        let ptr = g2_bytes.as_ptr() as *const [[u8; BYTES_PER_G2_POINT]; NUM_G2_POINTS];
        &*ptr
    };

    let s2 = KzgSettings::load_trusted_setup(g1_ref, g2_ref).unwrap();
    env::commit(&input);
}
