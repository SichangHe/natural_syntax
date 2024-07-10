# Natural Language Syntax Highlighting

WIP

The goal is to make a language server that highlights different parts of
speech in plain text, starting from English.

## Installation

Note that, by default, we download `libtorch` for you;
to use manual installation or use a version with CUDA support,
please see Rust-BERT's documentation.[^download-torch]

## Prediction Scheduling

For a single document, only one prediction is scheduled at a time.
When a prediction is ongoing,
new updates are queued and
the latest update replaces any previous updates queued.

[^download-torch]: [Automatic installation |
`rust_bert`](https://docs.rs/rust-bert/0.22.0/rust_bert/#automatic-installation)
