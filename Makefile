all:
	test production

test:
	cargo test

production:
	cargo build --release

dev:
	cargo build
	sudo mv target/debug/iridium /usr/local/bin
	sudo chmod ugo+x /usr/local/bin/iridium