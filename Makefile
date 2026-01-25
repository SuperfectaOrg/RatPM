PREFIX ?= /usr
BINDIR = $(PREFIX)/bin
SYSCONFDIR = /etc
UNITDIR = $(PREFIX)/lib/systemd/system
MANDIR = $(PREFIX)/share/man
CACHEDIR = /var/cache/ratpm
LIBDIR = /var/lib/ratpm

CARGO = cargo
INSTALL = install
PANDOC = pandoc

.PHONY: all build release test clean install uninstall man check fmt clippy

all: build

build:
	$(CARGO) build

release:
	$(CARGO) build --release

test:
	$(CARGO) test

check:
	$(CARGO) check

fmt:
	$(CARGO) fmt --all -- --check

clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

man: docs/ratpm.8 docs/ratpm.toml.5

docs/ratpm.8: docs/ratpm.8.md
	$(PANDOC) -s -t man docs/ratpm.8.md -o docs/ratpm.8

docs/ratpm.toml.5: docs/ratpm.toml.5.md
	$(PANDOC) -s -t man docs/ratpm.toml.5.md -o docs/ratpm.toml.5

clean:
	$(CARGO) clean
	rm -f docs/ratpm.8 docs/ratpm.toml.5

install: release man
	$(INSTALL) -D -m 755 target/release/ratpm $(DESTDIR)$(BINDIR)/ratpm
	$(INSTALL) -D -m 644 ratpm.toml.example $(DESTDIR)$(SYSCONFDIR)/ratpm/ratpm.toml
	$(INSTALL) -D -m 644 systemd/ratpm-refresh.service $(DESTDIR)$(UNITDIR)/ratpm-refresh.service
	$(INSTALL) -D -m 644 systemd/ratpm-refresh.timer $(DESTDIR)$(UNITDIR)/ratpm-refresh.timer
	$(INSTALL) -D -m 644 docs/ratpm.8 $(DESTDIR)$(MANDIR)/man8/ratpm.8
	$(INSTALL) -D -m 644 docs/ratpm.toml.5 $(DESTDIR)$(MANDIR)/man5/ratpm.toml.5
	$(INSTALL) -d -m 755 $(DESTDIR)$(CACHEDIR)
	$(INSTALL) -d -m 755 $(DESTDIR)$(LIBDIR)

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/ratpm
	rm -rf $(DESTDIR)$(SYSCONFDIR)/ratpm
	rm -f $(DESTDIR)$(UNITDIR)/ratpm-refresh.service
	rm -f $(DESTDIR)$(UNITDIR)/ratpm-refresh.timer
	rm -f $(DESTDIR)$(MANDIR)/man8/ratpm.8
	rm -f $(DESTDIR)$(MANDIR)/man5/ratpm.toml.5
	rm -rf $(DESTDIR)$(CACHEDIR)
	rm -rf $(DESTDIR)$(LIBDIR)

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all       - Build debug version (default)"
	@echo "  build     - Build debug version"
	@echo "  release   - Build optimized release version"
	@echo "  test      - Run tests"
	@echo "  check     - Run cargo check"
	@echo "  fmt       - Check code formatting"
	@echo "  clippy    - Run clippy lints"
	@echo "  man       - Generate man pages from markdown"
	@echo "  clean     - Remove build artifacts"
	@echo "  install   - Install to system (requires root)"
	@echo "  uninstall - Remove from system (requires root)"
	@echo "  help      - Show this help message"
