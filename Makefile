default:
	cd program && cargo prove build
	cd script && RUST_LOG=debug cargo run --release