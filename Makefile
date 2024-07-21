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

fetch-css-reset:
	curl -Lo reset.css "https://unpkg.com/reset-css/reset.css" # meyer reset - https://github.com/shannonmoeller/reset-css

ws: watch-server
