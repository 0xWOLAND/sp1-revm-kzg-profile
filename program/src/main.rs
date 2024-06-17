#![no_main]
sp1_zkvm::entrypoint!(main);

use hex;
use revm::precompile::Error;
use revm::{
    precompile::kzg_point_evaluation::{run, VERSIONED_HASH_VERSION_KZG},
    primitives::{Bytes, Env, PrecompileResult},
};
use sha2::{Digest, Sha256};

fn read_hex() -> Vec<u8> {
    let data = sp1_zkvm::io::read::<String>();
    hex::decode(&data).unwrap()
}

#[sp1_derive::cycle_tracker]
fn profile_run(input: &Bytes, gas: u64, env: &Env) -> PrecompileResult {
    let output = run(input, gas, env)?;
    Ok(output)
}

pub fn main() {
    let data = read_hex();
    let commitment = hex::decode(data).unwrap();
    let mut versioned_hash = Sha256::digest(&commitment).to_vec();
    versioned_hash[0] = VERSIONED_HASH_VERSION_KZG;

    let z = read_hex();
    let y = read_hex();
    let proof = read_hex();

    let gas = sp1_zkvm::io::read::<u64>();
    let env = Env::default();

    let input = [versioned_hash, z, y, commitment, proof].concat();
    let (gas, bytes) = profile_run(&input.into(), gas, &env).unwrap();

    sp1_zkvm::io::commit(&gas);
    sp1_zkvm::io::commit(&hex::encode(bytes));
}
