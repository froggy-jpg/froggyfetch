all: install

install:
	cargo build --release
	sudo cp ./target/release/froggyfetch /usr/bin/froggyfetch