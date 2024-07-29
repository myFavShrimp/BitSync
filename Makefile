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

static-assets: fetch-static-assets font-css

fetch-static-assets:
	nu ./scripts/make_fetch_static_assets.nu --all

font-css:
	nu ./scripts/make_font_css.nu

templates:
	nu ./scripts/make_templates.nu

ws: watch-server
