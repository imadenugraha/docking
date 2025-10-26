BINARY_NAME := docking

INSTALL_PATH := /usr/bin

install-release:
	@echo "Building the docking..."
	cargo build --release

	@echo "Moving docking binary to $(INSTALL_PATH)"
	sudo mv target/release/$(BINARY_NAME) $(INSTALL_PATH)/$(BINARY_NAME)

install-debug:
	@echo "Building the docking..."
	cargo build
	mv target/debug/$(BINARY_NAME) .
