# code-snap WASM Library

This library exposes a WebAssembly interface for generating image snapshots of code snippets with customizable syntax highlighting themes and backgrounds.

## Installation

You can install the `code-snap` WASM package via npm:

```bash
npm install code-snap
```

Or, if you are using Bun, install with:

```bash
bun add code-snap
```

or with yarn:

```bash
yarn add code-snap
```

## Overview

The primary function in this crate is `codesnap`, which accepts source code, a language identifier, and an optional JSON configuration string, and returns an image snapshot of the rendered code snippet.

## Usage

### `codesnap` function

```typescript
/**
 * Generates a code snapshot image.
 *
 * @param code - The source code string.
 * @param language - Optional language identifier for syntax highlighting.
 * @param config - Optional configuration object for options (e.g. theme, background).
 * @returns An object with `width`, `height`, and `data` properties where `data` is a Uint8Array of PNG bytes.
 */
function codesnap(code: string, language?: string, config?: object): ImageData;
```

- **Parameters:**
  - `code`: The source code snippet to render.
  - `language` (optional): The programming language of the code (used for syntax highlighting).
  - `config` (optional): Configuration object for customizing theme, background, scale factor, etc.

- **Returns:**
  - `ImageData` struct containing:
    - `width`: Width of the generated image.
    - `height`: Height of the generated image.
    - `data`: A vector of bytes representing the PNG image data.

- **Description:**
  Generates a syntax-highlighted image snapshot of the provided code string. Uses a default theme and background if no configuration is provided. The configuration can override theme, background color, and other options.

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
import { codesnap } from "code-snap";

const code = "console.log('Hello, world!');";

// Example with minimal parameters
const image1 = codesnap(code);

console.log(`Image1 dimensions: ${image1.width}x${image1.height}`);
console.log(`Image1 data length: ${image1.data.length}`);

// Example with language and config
const image2 = codesnap(code, "javascript", {
  theme: "candy",
  background: "#000000",
  scale_factor: 1,
});

console.log(`Image2 dimensions: ${image2.width}x${image2.height}`);
console.log(`Image2 data length: ${image2.data.length}`);
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
