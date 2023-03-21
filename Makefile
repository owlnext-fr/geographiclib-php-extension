all: test release

build:
	@cargo build

release:
	@cargo build --release

test:
	@cargo test

test-extension: release
	@php -d extension=./target/release/libgeographiclib_php_extension.so ./tests/test.php

install:
	@if [ ! -f "$$HOME/.cargo/bin/cargo-php" ]; then cargo install cargo-php; fi;
	@cargo php install --release --yes

uninstall:
	@if [ ! -f "$$HOME/.cargo/bin/cargo-php" ]; then cargo install cargo-php; fi;
	@cargo php remove --yes

stubs:
	@if [ ! -f "$$HOME/.cargo/bin/cargo-php" ]; then cargo install cargo-php; fi;
	@cargo php stubs

clean:
	@rm -rf target
	@rm -rf geographiclib/*.o
	@rm -rf geographiclib/*.a
