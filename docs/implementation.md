# implementation
(authored by agents unless marked 🧑)

- goal
  - 🧑 “fix natural_syntax_ls. It’s currently hard to install. And make a JS version of the idea.”
- Rust model
  - `tract-onnx` runs MobileBERT without external libtorch libraries
  - Hugging Face files are downloaded once and cached
  - pinned revision keeps model labels and tokenizer consistent
  - tokenize without special tokens, split into at most 510 subwords, surround each chunk with CLS and SEP
  - softmax confidence and label come from the first subword of each word
  - merge subwords using tokenizer word IDs
  - convert byte offsets to Unicode scalar offsets for the existing server
  - chunk boundaries limit context; English predictions are estimates
- server
  - existing token mapping, filtering and document scheduling remain
  - convert Unicode scalar offsets to UTF-16 units for LSP positions and lengths
  - prediction errors produce empty highlighting and complete processing so queued edits can proceed
- verification
  - `cargo test --workspace` exercises predictions, long input and Unicode offsets
  - `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt -- --check`
  - `cargo check -p natural_syntax --no-default-features` checks optional derives
  - `cargo install --path natural_syntax_ls` then `node scripts/check-lsp.mjs` tests installed stdio server
    - initialization, Unicode and CRLF highlighting, empty edit, shutdown
- JavaScript
  - see `natural_syntax_js/README.md`
