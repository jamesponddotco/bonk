.POSIX:
.SUFFIXES:

PREFIX = /usr/local
BINDIR = bin
TARGET = target/release
MANDIR = share/man

CARGO = cargo
RUSTFMT = rustfmt
GIT = git
RM = rm
INSTALL = install
SCDOC = scdoc

all: build doc

pre-commit: fmt build clean # Runs all pre-commit checks.

commit: pre-commit # Commits the changes to the repository.
	$(GIT) commit -s

push: commit # Pushes the changes to the repository.
	$(GIT) push origin trunk

build: # Builds an application binary.
	$(CARGO) fetch --locked
	$(CARGO) build --frozen --release --all-features

doc: # Builds the manpage.
	$(SCDOC) <doc/bonk.1.scd >bonk.1

install: # Installs the release binary.
	$(INSTALL) -d \
		$(DESTDIR)$(PREFIX)/$(BINDIR)/ \
		$(DESTDIR)$(PREFIX)/$(MANDIR)/man1/
	$(INSTALL) -pm 0755 $(TARGET)/bonk $(DESTDIR)$(PREFIX)/$(BINDIR)/
	$(INSTALL) -pm 0644 bonk.1 $(DESTDIR)$(PREFIX)/$(MANDIR)/man1/

fmt: # Checks Rust source files in the repository for formatting issues.
	$(RUSTFMT) --check src/*.rs

clean: # Cleans any build output.
	$(RM) -rf target bonk.1

.PHONY: all pre-commit commit push build doc install fmt clean
