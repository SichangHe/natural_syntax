# Natural Language Syntax Highlighting

WIP

The goal is to make a language server that highlights different parts of
speech in plain text, starting from English.

## Installation

Please download `libtorch` and specify the `LIBTORCH` and
`LD_LIBRARY_PATH` variables following Rust-BERT's
documentation.[^download-torch]
You could also use automatic installation if
you can enable the `rust-bert-download-libtorch` feature like the command below
does, but the binary built cannot find `libtorch` by default (sucker!).

```sh
cargo install natural-language-ls -F=rust-bert-download-libtorch
```

<details>
<summary>To install <code>libtorch</code> on macOS and
specify the environment variables.</summary>

<!-- Source: <https://github.com/guillaume-be/rust-bert/issues/326#issuecomment-1468703653> -->

```sh
brew install pytorch jq
export LIBTORCH=$(brew --cellar pytorch)/$(brew info --json pytorch | jq -r '.[0].installed[0].version')
export LD_LIBRARY_PATH=$LIBTORCH/lib:$LD_LIBRARY_PATH
```

Or, in fish:

```fish
brew install pytorch jq
set -gx LIBTORCH (brew --cellar pytorch)/(brew info --json pytorch | jq -r '.[0].installed[0].version')
set -gx LD_LIBRARY_PATH $LIBTORCH/lib $LD_LIBRARY_PATH
```

</details>

[^download-torch]: [Manual installation (recommended) |
`rust_bert`](https://docs.rs/rust-bert/0.22.0/rust_bert/#manual-installation-recommended)
