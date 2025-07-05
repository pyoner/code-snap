# Makefile for building WebAssembly (wasm)

# Variables
WASM_TARGET=web
PKG_DIR=pkg
SRC_TS=index.ts
DIST_TS=$(PKG_DIR)/$(SRC_TS)
# Add scope variable for wasm-pack build command
SCOPE = pyoner

# Default target builds wasm, TypeScript, and updates package.json metadata.
all: build-wasm update-package-json

# Remove pkg directory and its contents
clean:
	rm -rf $(PKG_DIR)

# Build wasm binaries for web target using wasm-pack
build-wasm:
	wasm-pack build --target $(WASM_TARGET) --scope $(SCOPE)

# Compile TypeScript source to JavaScript and declaration files in pkg directory
build-ts:
	tsc -p tsconfig.json --outDir $(PKG_DIR)

# Update package.json fields: files, main, and types in pkg directory
# Depends on the TypeScript build outputs to exist in pkg
update-package-json: build-ts
	@files=$$(ls $(PKG_DIR)/index.* 2>/dev/null | xargs -n1 basename | jq -R . | jq -s .); \
	if [ "$$files" = "[]" ]; then \
		echo "No index files found to add to package.json files field."; \
		exit 0; \
	fi; \
	package_json=$(PKG_DIR)/package.json; \
	tmp_file=$${package_json}.tmp; \
	jq --argjson files "$$files" '.files as $$oldFiles | .files = ($$oldFiles + $$files | unique) | .main = "index.js" | .types = "index.d.ts"' $$package_json > $$tmp_file && mv $$tmp_file $$package_json && echo "Updated package.json files field and main/types with: $$(echo $$files | jq -c .)"

publish:
	cd pkg && npm publish --access=public

.PHONY: all build-wasm build-ts update-package-json clean publish
