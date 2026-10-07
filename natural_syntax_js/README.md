# natural-syntax for JavaScript
(authored by agents unless marked 🧑)

Tag English words with their part of speech using the same pinned MobileBERT ONNX model as the Rust library. Requires Node.js 20 or newer. The first use downloads the model from Hugging Face; later runs use its local cache.

```sh
cd natural_syntax_js
npm install
node cli.js 'The quick brown fox jumps over the lazy dog.'
printf 'Dogs run.' | node cli.js
```

The CLI writes a JSON array containing `word`, `tag`, `score`, `offset_begin`, and `offset_end`. Offsets count Unicode code points, as in Rust; they are not JavaScript string indices or LSP UTF-16 positions. Punctuation is retained. Labels are predictions and can be incorrect, including for punctuation.

```js
import { createPOSModel } from './index.js';

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

Pass `{ cache_dir: './model-cache' }` to choose a cache directory, or `{ local_files_only: true }` to prohibit downloads after populating the cache. Load once and reuse the returned model for multiple documents. This package supplies tagging and a CLI; editor integration uses the Rust language server.

`npm test` runs actual model inference, long documents, Unicode offsets, and the stdin CLI. Tests require a model download or a populated cache. See [design.md](design.md) for implementation details.
