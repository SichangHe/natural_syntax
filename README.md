# Natural Language Syntax Highlighting
(authored by human unless marked 🤖)

Natural-Syntax-LS is a language server that highlights different parts of
speech (POS) in plain text.

| Full highlighting                                                                                     | Partially disabled (via customization)                                                                 |
| ----------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| ![Full highlighting](https://github.com/user-attachments/assets/f8937b78-854b-490a-8962-ea076beea235) | ![Partially disabled](https://github.com/user-attachments/assets/86af4cc1-0db4-4f4e-893e-582b9c91e7a5) |

## Installation

🤖 In a checkout containing the ONNX implementation, install with a current
stable Rust toolchain (Rust 1.91 or newer) and its platform linker:

```sh
cargo install --path natural_syntax_ls
```

🤖 The older crates.io release still needs libtorch. This implementation runs ONNX
in Rust and requires no libtorch download or library-path configuration.

🤖 On first launch, the server downloads approximately 100 MB of English
MobileBERT model files from Hugging Face. Subsequent launches reuse the local
cache. Allow the first launch to finish before requesting highlighting.
The model cache follows `HF_HOME`, defaulting to `~/.cache/huggingface`.

🤖 A JavaScript implementation is available in
[natural_syntax_js](natural_syntax_js/README.md).

## Editor setup

### ✅ NeoVim setup with LSPConfig

Please paste the below `register_natural_syntax_ls` function in
your Nvim configuration, call it,
and set up `natural_syntax_ls` like any other LSPConfig language server.
[Please see my config for an
example](https://github.com/SichangHe/.config/blob/a01e81bb84dd24ef350882e912d56feb1c3ef9db/nvim/lua/plugins/lsp.lua#L257).

<details><summary>The <code>natural_syntax_ls_setup</code> function.</summary>

```lua
local function natural_syntax_ls_setup(capabilities)
    require('lspconfig.configs').natural_syntax_ls = {
        default_config = {
            cmd = { 'natural-syntax-ls' },
            filetypes = { 'text' },
            single_file_support = true,
        },
        docs = {
            description = [[The Natural Syntax Language Server for highlighting parts of speech.]],
        },
    }
end
```

You can customize by setting `init_options` when calling the setup function:

```lua
require('lspconfig')['natural_syntax_ls'].setup {
    init_options = {
        token_map_update = { -- Customize your POS-token mapping here. E.g.:
            -- Disable coordinating conjunctions highlighting.
            CC = vim.NIL, -- `nil` does not work because it gets ignored.
            -- Highlight wh-determiners as enum members without any modifiers.
            WDT = { type = "enumMember" },
            -- Highlight determiners as read-only classes.
            DT = { type = "class", modifiers = { "readonly" } },
        },
    },
}
```

</details>

Customizations:

- I only set the `filetypes` field to `text`,
    but you can enable natural-syntax-ls for any other file types as well.
    Note that, though,
    the language server's semantic tokens supersede Tree-sitter highlighting by
    default.
- By specifying the `token_map_update` field in `init_options`,
    you can customize the mapping between parts of speech and semantic tokens.
    - The default mapping is in the `pos2token_bits` function in
        [`semantic_tokens.rs`][semantic_tokens.rs].
    - Part of speech tags are the variants of the `PartOfSpeech` enum in
        [`lib.rs`](https://github.com/SichangHe/natural_syntax/blob/main/src/lib.rs).
    - Token types and modifiers are variants of `TokenType` and
        `TokenModifier` in [`semantic_tokens.rs`][semantic_tokens.rs],
        all in camelCase.

### ❓ Visual Studio Code and other editor setup

<details>
<summary>No official support, but community plugins are welcome.</summary>

I do not currently use VSCode and these other editors,
so I do not wish to maintain plugins for them.

However,
it should be straightforward to implement plugins for them since
Natural-Syntax-LS implements the Language Server Protocol (LSP).
So,
please feel free to make a plugin yourself and create an issue for me to
link it here.

</details>

## Selected specification

### Prediction Scheduling

For a single document, only one prediction is scheduled at a time.
When a prediction is ongoing,
new updates are queued and
the latest update replaces any previous updates queued.

## Debugging

We use `tracing-subscriber` with the `env-filter` feature to
emit logs[^tracing-env-filter].
Please configure the log level by setting the `RUST_LOG` environment variable.

🤖 The ONNX implementation requires no `DYLD_LIBRARY_PATH` setting.

## Future work

- [ ] Customizing the mapping between part of speech and semantic token.
- [ ] Support languages other than English. This simply requires a new model.
- [ ] Incremental updates and semantic token ranges.
- [ ] Do not overwrite Markdown/LaTeX syntax highlighting.

[^tracing-env-filter]: <https://docs.rs/tracing-subscriber/latest/tracing_subscriber/#feature-flags>

[semantic_tokens.rs]: https://github.com/SichangHe/natural_syntax/blob/main/natural_syntax_ls/src/semantic_tokens.rs
