# Makefile for building WebAssembly (wasm)

# Variables
WASM_TARGET=web
PKG_DIR=pkg
SRC_TS=index.ts
DIST_TS=$(PKG_DIR)/$(SRC_TS)

# Default target builds wasm, TypeScript, and updates package.json metadata.
all: wasm-build update-package-json

# Remove pkg directory and its contents
clean:
	rm -rf $(PKG_DIR)

# Build wasm binaries for web target using wasm-pack
wasm-build:
	wasm-pack build --target $(WASM_TARGET)

# Copy the TypeScript source file to pkg directory
copy-ts:
	cp $(SRC_TS) $(DIST_TS)

# Compile TypeScript source to JavaScript and declaration files in pkg directory
ts-build:
	tsc -p tsconfig.json --outDir $(PKG_DIR)

# Update package.json fields: files, main, and types in pkg directory
# Depends on the TypeScript build outputs to exist in pkg
update-package-json: ts-build
	@files=$$(ls $(PKG_DIR)/index.* 2>/dev/null | xargs -n1 basename | jq -R . | jq -s .); \
	if [ "$$files" = "[]" ]; then \
		echo "No index files found to add to package.json files field."; \
		exit 0; \
	fi; \
	package_json=$(PKG_DIR)/package.json; \
	tmp_file=$${package_json}.tmp; \
	jq --argjson files "$$files" '.files as $$oldFiles | .files = ($$oldFiles + $$files | unique) | .main = "index.js" | .types = "index.d.ts"' $$package_json > $$tmp_file && mv $$tmp_file $$package_json && echo "Updated package.json files field and main/types with: $$(echo $$files | jq -c .)"

.PHONY: all wasm-build copy-ts ts-build update-package-json clean
