install:
	which cargo-binstall >/dev/null || cargo install cargo-binstall
	cargo binstall --no-confirm trunk
	cargo binstall --no-confirm basic-http-server

pack module:
    cd {{module}} && trunk build --release --public-url ./ --dist ../static/module/{{module}}

serve module:
    cd {{module}} && trunk serve

build:
    just pack square
    just pack yew-app

run:
	basic-http-server static
