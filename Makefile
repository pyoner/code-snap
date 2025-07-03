# Makefile for building wasm and bundling with bun

# Variables
WASM_TARGET=web
PKG_DIR=pkg
SRC_TS=auto-init.ts
DIST_TS=$(PKG_DIR)/$(SRC_TS)

# Default target
all: wasm-build ts-build

# Build wasm with wasm-pack for web target
wasm-build:
	wasm-pack build --target $(WASM_TARGET)

# Copy the TypeScript file to pkg directory
copy-ts:
	cp $(SRC_TS) $(DIST_TS)

# Run tsc to transpile auto-init.ts to JS without bundling wasm using tsconfig
ts-build: copy-ts
	tsc -p tsconfig.json --noResolve --outDir $(PKG_DIR)

.PHONY: all wasm-build copy-ts ts-build
