run:
	wasm-pack --log-level warn build --dev --target web ./wasm && cargo run

watch:
	cargo watch -s 'wasm-pack --log-level warn build --dev --target web ./wasm && cargo run'

# Files are served at localhost:3000.
