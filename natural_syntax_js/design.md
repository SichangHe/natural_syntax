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

validation
- `test.js` runs the actual pinned graph
- check familiar English tags, punctuation, multi-subword words, Unicode source spans, and text beyond one model window
- exercise stdin and help through the CLI process
