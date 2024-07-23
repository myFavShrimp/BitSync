.PHONY: *

watch-server:
	cargo watch -x run

link-env:
	ln -s dev.env .env

init-db:
	sqlx database create
	sqlx migrate run

lint:
	cargo check
	cargo clippy
	cargo fmt --check

fmt:
	cargo fmt

install-tools:
	cargo install sqlx-cli
	cargo install cargo-watch
	cargo install minhtml
	cargo install nu

fetch-css-reset:
	curl -Lo assets/reset.css "https://unpkg.com/reset-css/reset.css" # meyer reset - https://github.com/shannonmoeller/reset-css

fetch-mdui:
	curl -Lo assets/mdui.css "https://unpkg.com/mdui@2/mdui.css"
	curl -Lo assets/mdui.global.js "https://unpkg.com/mdui@2/mdui.global.js"

templates:
	nu ./build_html.nu

ws: watch-server
