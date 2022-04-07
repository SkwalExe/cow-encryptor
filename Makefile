all: build

build:
	cargo build --release

install:
	sudo cp target/release/cow-encryptor /usr/bin/cow-encryptor

uninstall:
	sudo rm -f /usr/bin/cow-encryptor