all: cargo

cargo:
		cargo build --release

install:
		sudo cp ./target/release/froggyfetch /usr/bin/froggyfetch