CLI_NAME = dman
GUI_NAME = dman-gui

all: build

build:
	@echo "Building CLI..."
	cargo build --release

build-gui:
	@echo "Building CLI and GUI..."
	cargo build --release --features gui

install: build
	@echo "Installing CLI..."
	sudo install -Dm755 target/release/$(CLI_NAME) /usr/bin/$(CLI_NAME)

install-gui: build-gui
	@echo "Installing CLI and GUI..."
	sudo install -Dm755 target/release/$(CLI_NAME) /usr/bin/$(CLI_NAME)
	sudo install -Dm755 target/release/$(GUI_NAME) /usr/bin/$(GUI_NAME)
