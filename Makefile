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
	cargo install --locked nu

static-assets: fetch-static-assets font-css

fetch-static-assets:
	nu ./scripts/make_fetch_static_assets.nu --all

font-css:
	nu ./scripts/make_font_css.nu

ws: watch-server
