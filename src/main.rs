#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;
use c_kzg::{Blob, BYTES_PER_BLOB, BYTES_PER_COMMITMENT, BYTES_PER_PROOF, BYTES_PER_G1_POINT, BYTES_PER_G2_POINT, FIELD_ELEMENTS_PER_BLOB, KzgSettings};
use c_kzg::{Bytes48};
use c_kzg::{KzgProof};
use c_kzg::bindings::blst_fr;
mod trusted_setup;
use trusted_setup::{g1_bytes, g2_bytes};
use core::slice;
const NUM_G2_POINTS: usize = 65;
//use kzg::setup::Setup;

risc0_zkvm::guest::entry!(main);


fn main() {
    // TODO: Implement your guest code here

    // read the input
    let mut blob = [0; BYTES_PER_BLOB];
    env::read_slice(&mut blob[..]);
    let mut commitment = [0; BYTES_PER_COMMITMENT];
    env::read_slice(&mut commitment[..]);
    let commitment_bytes = Bytes48::from_bytes(&commitment).unwrap();
    let mut proof = [0; BYTES_PER_PROOF];
    env::read_slice(&mut proof[..]);
    let proof_bytes = Bytes48::from_bytes(&proof).unwrap();

    let b = Blob::from_bytes(&blob).unwrap();
    /*let cmt = KzgCommitment::from_bytes(&commitment).unwrap();
    let p = KzgProof::from_bytes(&proof).unwrap();*/

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

    let mut buf1 = [[0u8; 32]];
    let mut buf2 = [[0u8; 144]];
    let mut buf3 = [blst_fr::default(); 4097];
    let mut buf4 = [blst_fr::default(); 4097];
    let mut buf5 = [blst_fr::default(); 4097];
    let s2 = KzgSettings::load_trusted_setup(g1_ref, g2_ref, &mut buf1, &mut buf2, &mut buf3).unwrap();

    let result = KzgProof::verify_blob_kzg_proof(&b, &commitment_bytes, &proof_bytes, &s2, &mut buf4, &mut buf5).unwrap();
    
    env::commit(&result);
}
