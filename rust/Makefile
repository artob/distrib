CARGO = cargo
READMER = readmer

all: README.md

README.md: ../.config/readmer/rust/README.md.liquid Cargo.toml lib/distrib/Cargo.toml
	cd lib/distrib && $(READMER) render ../../../.config/readmer/rust/README.md.liquid > ../../README.md.tmp
	mv README.md.tmp README.md

build: Cargo.toml
	$(CARGO) build --release

check: Cargo.toml
	$(CARGO) test -- --nocapture

clean: Cargo.toml
	rm -rf *~ target
	$(CARGO) clean

maintainer-clean: Cargo.toml
	rm -f Cargo.lock

.PHONY: all check clean maintainer-clean
.SECONDARY:
.SUFFIXES:
