CARGO ?= cargo

PREFIX ?= $(HOME)/.bin
INSTALL_NAME ?= harpoon

.PHONY: all build check fmt fmt-check lint lint-fix test clean install

all: check build

build:
	$(CARGO) build --release

check:
	$(CARGO) check --all-targets --all-features

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

lint-fix:
	$(CARGO) clippy --fix --allow-dirty --allow-staged --all-targets --all-features -- -D warnings

test:
	$(CARGO) test --all-targets --all-features

clean:
	$(CARGO) clean

ifeq ($(OS),Windows_NT)
install:
	$(CARGO) install --path . --force
else
install: build
	install -d $(PREFIX)
	install -m 755 target/release/harpoon $(PREFIX)/$(INSTALL_NAME)
endif
