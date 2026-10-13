# natural-syntax for JavaScript
(authored by agents unless marked 🧑)

Tag English text in the browser using the same pinned MobileBERT ONNX model as the Rust library. Inference runs locally through WebAssembly. The first use downloads the model from Hugging Face; later visits use the browser cache. A Node.js CLI and language server are also available.

## Browser

```sh
npm install natural-syntax
```

Import `createPOSModel` in your browser application using a bundler such as Vite. The library example below works in the browser without changes. Load the model once and reuse it; the full-precision model download is approximately 100 MB. Browsers must support WebAssembly and BigInt.

To run the included browser page from this repository:

```sh
cd natural_syntax_js
npm install
npm run browser
```

The page accepts text and displays words, predicted tags, scores, and offsets. Node.js is required for the development server; tagging runs in the browser.

## Language server

```sh
npm install -g ./natural_syntax_js # From the repository folder.
```

This installs `natural-syntax-ls-js`, which speaks the Language Server Protocol over stdio exactly like the Rust `natural-syntax-ls`: same semantic token legend, default highlighting, and `token_map_update` initialization option. Use any editor setup from the [main README](../README.md) with the command replaced, e.g. set `naturalSyntaxLs.serverPath` to `natural-syntax-ls-js` in the VS Code extension.

## CLI and library

```sh
natural-syntax 'The quick brown fox jumps over the lazy dog.'
printf 'Dogs run.' | natural-syntax
```

The CLI writes a JSON array containing `word`, `tag`, `score`, `offset_begin`, and `offset_end`. Offsets count Unicode code points, as in Rust; they are not JavaScript string indices or LSP UTF-16 positions. Punctuation is retained. Labels are predictions and can be incorrect, including for punctuation.

```js
import { createPOSModel } from 'natural-syntax';

const loaded = await createPOSModel();
if (!loaded.ok) throw new Error(loaded.error);
try {
    const result = await loaded.value.predict('Dogs run.');
    if (!result.ok) throw new Error(result.error);
    console.log(result.value);
} finally {
    await loaded.value.dispose();
}
```

In Node.js, pass `{ cache_dir: './model-cache' }` to choose a cache directory, or `{ local_files_only: true }` to prohibit downloads after populating the cache. These filesystem options do not apply to the browser. Load once and reuse the returned model for multiple documents.

`npm run test:browser` runs actual inference in headless Chromium, including Unicode offsets and long documents. Install Chromium with `npx playwright install chromium`, or set `CHROME_PATH` to an existing Chrome executable. `npm test` checks Node.js inference, semantic token conversion, and the stdin CLI. Tests require network access or a populated cache. See [design.md](design.md) for implementation details.
