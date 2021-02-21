build:
	cargo build

migration:
	make -C ./server migration

debug_server: run_server
	RUST_LOG=server=debug cargo run

run_server:
	docker-compose up -d
