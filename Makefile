install:
	@which wasm-pack >/dev/null || cargo install --locked wasm-pack
	@which basic-http-server >/dev/null || cargo install --locked basic-http-server

pack-square:
	wasm-pack build square --target web --out-dir static/module/square
	rm -f static/module/square/.gitignore

pack-ship:
	wasm-pack build ship --target web --out-dir static/module/ship
	rm -f static/module/ship/.gitignore

pack: pack-square pack-ship

serve:
	basic-http-server static
