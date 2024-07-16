# Natural Language Syntax Highlighting

Natural-Syntax-LS is a language server that highlights different parts of
speech (POS) in plain text.

| Full highlighting                                                                                     | Partially disabled (via customization)                                                                 |
| ----------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| ![Full highlighting](https://github.com/user-attachments/assets/f8937b78-854b-490a-8962-ea076beea235) | ![Partially disabled](https://github.com/user-attachments/assets/86af4cc1-0db4-4f4e-893e-582b9c91e7a5) |

## Installation

1. Download `libtorch` v2.1 as per
    [Rust-BERT's documentation][download-torch].
    <details><summary>Tips.</summary>

    You can figure out the URL to download `libtorch` [in tch-rs' build
    script](https://github.com/LaurentMazare/tch-rs/blob/5480d6fd4be12e748e0d87555db54a5f6e74edf2/torch-sys/build.rs#L311).
    The `LIBTORCH` variable should be the `torch/` directory.

    </details>
    <details><summary>Why automatic installation does not work.</summary>

    Rust-BERT has an "automatic installation" option that
    uses tch-rs' build script to download `libtorch`.
    However,
    the binary produced this way does not run because that `libtorch` is not on
    `LD_LIBRARY_PATH`.
    Alternatively, you could statically link `libtorch`,
    but that would
    [require you to download `libtorch` yourself][tch-static-linking] anyway.

    </details>
1. Install the `natural_syntax_ls` package with Cargo or friends to
    get the `natural-syntax-ls` binary:

    ```sh
    cargo install natural_syntax_ls --no-default-features
    ```

    Setting `--no-default-features` disables downloading `libtorch`
    (automatic installation).

    <details><summary>Why automatic installation is the default.</summary>

    Because otherwise it would be a pain to run the continuous integration.

    </details>

## Editor setup

### ✅ NeoVim setup with LSPConfig

Please paste the below `natural_syntax_ls_setup` function in
your Nvim configuration and call it with your client's `capabilities`.
[Please see my config for an
example](https://github.com/SichangHe/.config/blob/b0961205a060d3588f56e97fd066a35424fe64a9/nvim/lua/plugins/lsp.lua#L301).

<details><summary>The <code>natural_syntax_ls_setup</code> function.</summary>

```lua
local function natural_syntax_ls_setup(capabilities)
    local lspconfig = require('lspconfig')
    require('lspconfig.configs')['natural_syntax_ls'] = {
        default_config = {
            cmd = { 'natural-syntax-ls' },
            filetypes = { 'text' },
            single_file_support = true,
        },
        docs = {
            description = [[The Natural Syntax Language Server for highlighting parts of speech.]],
        },
    }
    lspconfig['natural_syntax_ls'].setup {
        capabilities,
        init_options = {
            token_map_update = {
                -- Customize your POS-token mapping here. E.g.:
                --[[
                -- Disable coordinating conjunctions highlighting.
                CC = vim.NIL, -- `nil` does not work because it gets ignored.
                -- Highlight wh-determiners as enum members without any modifiers.
                WDT = { type = "enumMember" },
                -- Highlight determiners as read-only classes.
                DT = { type = "class", modifiers = { "readonly" } },
                ]]
            },
        },
    }
end
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

On macOS, you may need to set `DYLD_LIBRARY_PATH` to run the tests.

## Future work

- [ ] Customizing the mapping between part of speech and semantic token.
- [ ] Support languages other than English. This simply requires a new model.
- [ ] Incremental updates and semantic token ranges.
- [ ] Do not overwrite Markdown/LaTeX syntax highlighting.

[^tracing-env-filter]: <https://docs.rs/tracing-subscriber/latest/tracing_subscriber/#feature-flags>

[download-torch]: https://docs.rs/rust-bert/0.22.0/rust_bert/#manual-installation-recommended
[semantic_tokens.rs]: https://github.com/SichangHe/natural_syntax/blob/main/natural_syntax_ls/src/semantic_tokens.rs
[tch-static-linking]: https://github.com/LaurentMazare/tch-rs/tree/v2.1?tab=readme-ov-file#static-linking
