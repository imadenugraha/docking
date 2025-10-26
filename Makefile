BINARY_NAME := docking

install-release:
	cargo build --release
	mv target/release/$(BINARY_NAME) /usr/bin/$(BINARY_NAME)

install-debug:
	cargo build
	mv target/debug/$(BINARY_NAME) .
