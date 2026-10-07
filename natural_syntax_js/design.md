# JavaScript tagging design
(authored by agents unless marked 🧑)

tag English text without compiling Rust
- Transformers.js loads the pinned MobileBERT ONNX graph at full precision on CPU
  - `BertForTokenClassification` supplies the matching three-input ONNX runner
  - the automatic token-classification factory does not register MobileBERT in Transformers.js 4.3.1
- split input at whitespace, punctuation, ASCII symbols, and Han characters
  - approximate BERT's word boundaries before encoding each word
  - preserve source spelling and code-point offsets
  - control characters embedded inside words may differ from Rust tokenization
- concatenate encoded words for contextual inference
  - process up to 510 subwords between the model's classification and separator tokens
  - retain each word's first subword label and softmax confidence
  - words spanning chunks retain their original span
- return expected loading, input, and inference errors as `{ok: false, error}`
  - successful operations return `{ok: true, value}`
- dispose the ONNX session when finished

language server `server.js`
- JSON-RPC over stdio written directly; no LSP library
- full-text sync; `textDocument/semanticTokens/full` only
- per document, one prediction at a time; the latest edit replaces queued ones
  - token requests wait until the latest text is tagged
- `tokens.js` is pure
  - legend and default tag → token map copied from the Rust server's `semantic_tokens.rs`; keep them equal
  - converts code-point offsets to LSP line and UTF-16 column
  - drops words scoring ≤ 1/3, ASCII-punctuation-only words, and disabled tags
- invalid `token_map_update` is reported with `window/logMessage` and ignored
- model load failure exits with status 1

validation
- `test.js` runs the actual pinned graph
- check familiar English tags, punctuation, multi-subword words, Unicode source spans, and text beyond one model window
- exercise stdin and help through the CLI process
