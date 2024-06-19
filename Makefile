default:
	cd program && CC="/Users/bannem/Library/Application Support/cargo-risczero/cpp/bin/riscv32-unknown-elf-gcc" CFLAGS_riscv32im_succinct_zkvm_elf="-march=rv32im -nostdlib" RUSTUP_TOOLCHAIN="succinct" RUSTFLAGS="-C passes=loweratomic -C link-arg=-Ttext=0x00200800 -C panic=abort" cargo build --target riscv32im-succinct-zkvm-elf  --release
	cd script && RUST_LOG=debug cargo run --release