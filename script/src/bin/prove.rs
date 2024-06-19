use sp1_sdk::utils;
use sp1_sdk::{ProverClient, SP1Stdin};
pub const ELF: &[u8] = include_bytes!("../../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();
    let mut stdin = SP1Stdin::new();

    const COMMITMENT: &str = "8f59a8d2a1a625a17f3fea0fe5eb8c896db3764f3185481bc22f91b4aaffcca25f26936857bc3a7c2539ea8ec3a952b7";
    const Z: &str = "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000000";
    const Y: &str = "1522a4a7f34e1ea350ae07c29c96c7e79655aa926122e95fe69fcbd932ca49e9";
    const PROOF: &str = "a62ad71d14c5719385c0686f1871430475bf3a00f0aa3f7b8dd99a9abc2160744faf0070725e00b60ad9a026a15b1a8c";
    const EXPECTED_OUTPUT: &str = "000000000000000000000000000000000000000000000000000000000000100073eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";
    const GAS: u64 = 50000;

    stdin.write(&COMMITMENT);
    stdin.write(&Z);
    stdin.write(&Y);
    stdin.write(&PROOF);
    stdin.write(&GAS);

    let client = ProverClient::new();
    let (mut public_values, _) = client.execute(ELF, stdin).expect("failed to prove");

    let gas = public_values.read::<u64>();
    let bytes = public_values.read::<String>();

    for _ in 0..4 {
        assert_eq!(gas, GAS);
        assert_eq!(bytes, EXPECTED_OUTPUT);
    }
}
