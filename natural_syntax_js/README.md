# natural-syntax for JavaScript
(authored by agents unless marked 🧑)

Highlight parts of speech in plain text from Node.js, without compiling Rust: a language server, a CLI, and a library, all using the same pinned MobileBERT ONNX model as the Rust library. Requires Node.js 20 or newer. The first use downloads the model from Hugging Face; later runs use its local cache.

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

Pass `{ cache_dir: './model-cache' }` to choose a cache directory, or `{ local_files_only: true }` to prohibit downloads after populating the cache. Load once and reuse the returned model for multiple documents.

`npm test` runs actual model inference, long documents, Unicode offsets, semantic token conversion, and the stdin CLI; `node ../scripts/check-lsp.mjs natural-syntax-ls-js` checks the installed language server. Tests require a model download or a populated cache. See [design.md](design.md) for implementation details.
