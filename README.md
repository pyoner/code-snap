# CodeSnap WASM Library

This library exposes a WebAssembly interface for generating image snapshots of code snippets with customizable syntax highlighting themes and backgrounds.

## Overview

The primary function in this crate is `codesnap`, which accepts source code, a language identifier, and an optional JSON configuration string, and returns an image snapshot of the rendered code snippet.

## Usage

### `codesnap` function

```typescript
/**
 * Generates a code snapshot image.
 *
 * @param code - The source code string.
 * @param language - The language identifier for syntax highlighting.
 * @param config - Optional JSON string for configuration options (e.g. theme, background).
 * @returns An object with `width`, `height`, and `data` properties where `data` is a Uint8Array of PNG bytes.
 */
function codesnap(code: string, language: string, config?: string): ImageData;
```

- **Parameters:**
  - `code`: The source code snippet to render.
  - `language`: The programming language of the code (used for syntax highlighting).
  - `config`: An optional JSON string for customizing theme and background.

- **Returns:**
  - `ImageData` struct containing:
    - `width`: Width of the generated image.
    - `height`: Height of the generated image.
    - `data`: A vector of bytes representing the PNG image data.

- **Description:**
  Generates a syntax-highlighted image snapshot of the provided code string. Uses a default theme if no configuration is provided. The configuration can override theme and background color.

### `ImageData` struct

```typescript
interface ImageData {
  width: number;
  height: number;
  data: Uint8Array;
}
```

- Represents the generated code snapshot image.
- The PNG image data can be accessed via the `data` getter method.

## Example

```typescript
const code = "console.log('Hello, world!');";
const language = "javascript";
const config = JSON.stringify({
  theme: "candy",
  background: "#000000",
});

const image: ImageData = codesnap(code, language, config);
console.log(`Image dimensions: ${image.width}x${image.height}`);
console.log(`Image data length: ${image.data.length}`);
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
