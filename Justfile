# Files are served at localhost:3000.

# mode is '--dev' or '--release'.
wasm mode:
	wasm-pack --log-level warn build {{mode}} --target web ./wasm 	

build: (wasm "--dev")
	cargo build

build-release: (wasm "--release")
	cargo build

run: (wasm "--dev")
	cargo run

run-notify: build notify-done
	cargo run

run-release: (wasm "--release")
	cargo run --release

watch:
	cargo watch -s 'just run'

watch-notify:
	cargo watch -s 'just run-notify'

notify-done:
	echo 'Done compiling.' | terminal-notifier

# Files are served at localhost:3000.
