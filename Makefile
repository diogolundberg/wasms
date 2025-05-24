install:
	which cargo-binstall >/dev/null || cargo install cargo-binstall
	cargo binstall --no-confirm trunk
	cargo binstall --no-confirm wasm-pack basic-http-server

pack-square:
	$(MAKE) -C square pack

serve-square:
	$(MAKE) -C square serve

pack: pack-square

serve:
	basic-http-server static
