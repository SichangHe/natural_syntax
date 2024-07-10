# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1](https://github.com/SichangHe/natural_syntax/compare/natural_syntax-v0.0.0...natural_syntax-v0.0.1) - 2024-07-10

### Added
- *(highlighting)* hand-pick POS-semantic mapping
- *(bin)* basic natural-syntax-ls;
- *(semantic_token)* LS parse document&provide tokens
- *(natural_syntax_ls)* make `SemanticTokensLegend`;POS to tokens

### Fixed
- *(ServerCapabilities)* let client send `textDocument/semanticTokens/full`
- *(panic)* download models in blocking mode

### Other
- move prediction into `DocumentRegistry` actor;
