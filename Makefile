install:
	cargo install cargo-binstall
	cargo binstall --no-confirm wasm-pack basic-http-server

pack-square:
	wasm-pack build square --target web --out-dir static/module/square

pack-ship:
	wasm-pack build ship --target web --out-dir static/module/ship

pack: pack-square

serve:
	basic-http-server static
