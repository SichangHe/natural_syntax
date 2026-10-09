# Changelog
(authored by agents unless marked 🧑)
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.3](https://github.com/SichangHe/natural_syntax/compare/natural_syntax-v0.0.2...natural_syntax-v0.0.3) - 2026-10-08

### Added

- *(js)* language server natural-syntax-ls-js
- *(vscode)* extension that starts natural-syntax-ls
- *(js)* part-of-speech tagging library and CLI
- [**breaking**] run the model with ONNX in Rust instead of libtorch

### Fixed

- *(ci)* skip unbuildable libtorch release baselines

### Other

- install from Git, JS server, VS Code extension
- bump sccache-action for the current GitHub cache service
- bump versions;fix cache
- register LSPConfig without capabilities
- screenshots
- typo

## [0.0.2](https://github.com/SichangHe/natural_syntax/compare/natural_syntax-v0.0.1...natural_syntax-v0.0.2) - 2024-07-12

### Added
- *(num-traits)* `PartOfSpeech` impl `FromPrimitive`&`ToPrimitive`

### Fixed
- *(torch)* use system libtorch

## [0.0.1](https://github.com/SichangHe/natural_syntax/compare/natural_syntax-v0.0.0...natural_syntax-v0.0.1) - 2024-07-11

### Added
- *(trait)*: mark `POSModel` `Send` and `Sync`

### Other
- explain basics&installation
- *(cargo)*: workspace
