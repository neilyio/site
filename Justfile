run:
	wasm-pack --log-level warn build --dev --target web ./wasm && cargo run

watch:
	cargo watch -s 'wasm-pack --log-level warn build --dev --target web ./wasm && cargo run'

run-release:
	wasm-pack --log-level warn build --release --target web ./wasm && cargo run --release
	

# Files are served at localhost:3000.
