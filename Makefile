# Makefile for building wasm and bundling with bun

# Variables
WASM_TARGET=web
PKG_DIR=pkg
SRC_TS=index.ts
DIST_TS=$(PKG_DIR)/$(SRC_TS)

# Default target
all: wasm-build ts-build update-package-files

# Build wasm with wasm-pack for web target
wasm-build:
	wasm-pack build --target $(WASM_TARGET)

# Copy the TypeScript file to pkg directory
copy-ts:
	cp $(SRC_TS) $(DIST_TS)

# Run tsc to transpile index.ts to JS without bundling wasm using tsconfig
ts-build: copy-ts
	tsc -p tsconfig.json --noResolve --outDir $(PKG_DIR)

# Update pkg/package.json files field to include auto-init.* files
update-package-json:
	@files=$$(ls $(PKG_DIR)/index.* 2>/dev/null | xargs -n1 basename | jq -R . | jq -s .); \
	if [ "$$files" = "[]" ]; then \
		echo "No index files found to add to package.json files field."; \
		exit 0; \
	fi; \
	package_json=$(PKG_DIR)/package.json; \
	tmp_file=$${package_json}.tmp; \
	jq --argjson files "$$files" '.files as $$oldFiles | .files = ($$oldFiles + $$files | unique)' $$package_json > $$tmp_file && mv $$tmp_file $$package_json && echo "Updated package.json files field with: $$(echo $$files | jq -c .)"

.PHONY: all wasm-build copy-ts ts-build update-package-json
