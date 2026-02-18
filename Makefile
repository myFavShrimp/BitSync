.PHONY: *

watch-server:
	cargo watch -x run

link-env:
	ln -s dev.env .env

init-db:
	sqlx database create
	cd bitsync-database && sqlx migrate run

lint:
	cargo check
	cargo clippy
	cargo fmt --check

fmt:
	cargo fmt

install-tools:
	cargo install --locked sqlx-cli
	cargo install --locked cargo-watch
	cargo install --locked arc-automation

static-assets:
	arc run -s local -t static_assets

fetch-static-assets:
	arc run -s local -t fetch_static_assets

font-css:
	arc run -s local -t generate_font_css

start-postgres:
	arc run -s local -t start_postgres

fetch-hyperstim:
	arc run -s local -t fetch_hyperstim

ws: watch-server
