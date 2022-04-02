all: install

install:
	cargo build --release
	sudo cp target/release/cow-encryptor /usr/bin/cow-encryptor

uninstall:
	sudo rm -f /usr/bin/cow-encryptor