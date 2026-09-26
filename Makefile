# ==============================================================================
# Production-Grade Multi-Target Rust Workspace Makefile
# Target Toolchain: Rust 1.98.0 / Cargo 1.98.0 (2026-08)
# ==============================================================================

# Force GNU Make features, fail-fast bash execution, and silent directory changes
SHELL := /bin/bash
.SHELLFLAGS := -eu -o pipefail -c
MAKEFLAGS += --warn-undefined-variables --no-print-directory

# Rebuild targets automatically if the Makefile itself changes
.EXTRA_PREREQS := $(MAKEFILE_LIST)

# --- Configurable Workspace Variables ---
BINARY_NAME ?= app
BUILD_DIR ?= target/release

# Multi-target cross-compilation target triples
TARGETS := \
	aarch64-linux-android \
	aarch64-pc-windows-msvc \
	aarch64-unknown-linux-gnu \
	aarch64-unknown-linux-musl \
	armv7-linux-androideabi \
	i686-linux-android \
	wasm32-unknown-unknown \
	x86_64-linux-android \
	x86_64-pc-windows-msvc \
	x86_64-unknown-linux-gnu \
	x86_64-unknown-linux-musl

# Auto-detect mold linker on Linux host to speed up local linking steps
HOST_OS := $(shell uname -s)
MOLD_FLAG := $(if $(filter Linux,$(HOST_OS)),$(if $(shell command -v mold 2>/dev/null),-C link-arg=-fuse-ld=mold,),)

# Target-specific CPU feature flags tailored per target architecture
X86_64_FLAGS := -C target-feature=+sha,+sse2,+ssse3,+sse4.1,+sse4.2,+popcnt
ARM64_FLAGS := -C target-feature=+sha2,+aes,+neon
ARMV7_FLAGS  := -C target-feature=+v7,+vfp3,+d16

# Apply mold linker ONLY to Linux targets (excluding Windows MSVC and Android targets)
LINUX_MOLD := $(if $(findstring linux-gnu,$@)$(findstring linux-musl,$@),$(MOLD_FLAG),)

# Dynamic calculation of target-specific RUSTFLAGS
SPECIFIC_FLAGS = $(strip \
    $(if $(findstring x86_64,$@),$(X86_64_FLAGS) $(LINUX_MOLD), \
    $(if $(findstring i686,$@),-C target-feature=+sse2, \
    $(if $(findstring aarch64,$@),$(ARM64_FLAGS) $(LINUX_MOLD), \
    $(if $(findstring armv7,$@),$(ARMV7_FLAGS),)))))

# Target routing: utilizes `cargo-xwin` for MSVC targets and native `cargo` for non-MSVC
BUILD_CMD = $(if $(findstring msvc,$@),cargo xwin build,cargo build)

.DEFAULT_GOAL := help

# --- Primary Build Targets ---
.PHONY: all build-all clean help test clippy fmt deny desktop linux android windows $(TARGETS)

all: build-all

build-all: $(TARGETS)

## Pattern rule for building individual target triples
$(TARGETS):
	@echo "==> Building $(BINARY_NAME) for target: $@..."
	RUSTFLAGS="$(SPECIFIC_FLAGS)" $(BUILD_CMD) --release --target $@

## Grouped platform target rules
desktop: $(filter %-linux-gnu %-linux-musl %-windows-msvc, $(TARGETS)) ## Build Linux & Windows Desktop binaries
linux: $(filter %-linux-gnu %-linux-musl, $(TARGETS))                 ## Build Linux x86_64 & AArch64 binaries (gnu/musl)
android: $(filter %-android %-androideabi, $(TARGETS))                 ## Build Android NDK targets (AArch64, ARMv7, x86/x86_64)
windows: $(filter %-windows-msvc, $(TARGETS))                         ## Build Windows MSVC binaries (x86_64 & AArch64) via cargo-xwin

# --- Quality Control & CI Verification Targets ---
fmt: ## Enforce formatting via rustfmt
	cargo fmt --workspace -- --check

clippy: ## Execute workspace Clippy checks with strict warnings enabled
	cargo clippy --workspace --all-targets --all-features -- -D warnings

deny: ## Perform security advisory, license, and duplicate dependency audits
	cargo deny check

test: ## Execute workspace test suites
	cargo test --workspace --all-targets

clean: ## Remove workspace build directory and artifacts
	cargo clean

help: ## Show this interactive target menu
	@echo "Usage: make [target]"
	@echo ""
	@echo "Compilation Targets:"
	@echo "  build-all    Build release binaries for all $(words $(TARGETS)) target triples"
	@echo "  desktop      Build Linux (gnu/musl) and Windows MSVC target binaries"
	@echo "  linux        Build Linux (x86_64 & AArch64) target binaries"
	@echo "  android      Build Android NDK target binaries"
	@echo "  windows      Build Windows MSVC target binaries using cargo-xwin"
	@echo "  clean        Remove target directory artifacts"
	@echo ""
	@echo "Quality Assurance Targets:"
	@echo "  fmt          Verify code formatting against rustfmt rules"
	@echo "  clippy       Run workspace Clippy lints with -D warnings"
	@echo "  deny         Audit dependencies with cargo-deny"
	@echo "  test         Run tests across the workspace"
