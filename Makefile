# ─── ICNS to ICO Build System ────────────────────────────────────────────────
# Usage:
#   make          - Build optimized release executable
#   make run      - Build and run the app
#   make test     - Run unit tests
#   make clean    - Remove build artifacts

BINARY   = icns-to-ico.exe
MINGW    = C:/msys64/mingw64/bin
VERSION ?= $(shell git describe --tags --always --dirty 2>NUL || echo dev)
COMMIT  ?= $(shell git rev-parse --short HEAD 2>NUL || echo none)

# Add MinGW to PATH if present (required for stable-x86_64-pc-windows-gnu toolchain)
export PATH := $(MINGW):$(PATH)

.PHONY: all run test clean

all:
	cargo build --release
	copy /y target\release\$(BINARY) $(BINARY) >nul
	@echo.
	@echo Build complete: $(BINARY)
	@echo Version: $(VERSION)  Commit: $(COMMIT)
	@for %%F in ($(BINARY)) do @echo Size: %%~zF bytes

run: all
	.\$(BINARY)

test:
	cargo test

clean:
	@if exist $(BINARY) del $(BINARY)
	@cargo clean
	@echo Clean complete.
