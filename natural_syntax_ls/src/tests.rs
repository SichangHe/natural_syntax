use std::time::Instant;

use super::*;

use insta::assert_debug_snapshot;
use tracing::Level;
use tracing_subscriber::EnvFilter;

#[test]
fn token_types() {
    init_tracing();
    let types = semantic_token_types();
    assert_debug_snapshot!(types);
}

#[test]
fn token_modifiers() {
    init_tracing();
    let modifiers = semantic_token_modifiers();
    assert_debug_snapshot!(modifiers);
}

#[test]
fn filtering_tokens() {
    assert!(filter_token(&dummy_token("word", 1.)));
    assert!(filter_token(&dummy_token("word,", 0.4)));
    assert!(!filter_token(&dummy_token("word", 0.3)));
    assert!(!filter_token(&dummy_token(",", 0.3)));
    assert!(!filter_token(&dummy_token(",", 1.)));
    assert!(!filter_token(&dummy_token("]", 1.)));
    assert!(!filter_token(&dummy_token("(", 1.)));
}

fn dummy_token(word: &str, score: f64) -> POSToken {
    POSToken {
        word: word.into(),
        score,
        ..Default::default()
    }
}

#[test]
fn utf16_semantic_positions() {
    let text = Rope::from_str("😀 The birds\r\n😀 sing");
    let tokens = [
        ("😀", 0, 1),
        ("The", 2, 5),
        ("birds", 6, 11),
        ("sing", 15, 19),
    ]
    .map(|(word, offset_begin, offset_end)| POSToken {
        word: word.into(),
        score: 1.,
        tag: PartOfSpeech::NN,
        offset_begin,
        offset_end,
    });
    let positions = semantic_tokens(&text, &tokens, &TokenMap::default())
        .into_iter()
        .map(|token| (token.delta_line, token.delta_start, token.length))
        .collect::<Vec<_>>();
    assert_eq!(positions, [(0, 0, 2), (0, 3, 3), (0, 4, 5), (1, 3, 4)]);
}

#[test]
fn convert_tokens() {
    init_tracing();
    let start = Instant::now();
    let model = POSModel::try_default().unwrap();
    info!("Took {}ms to load the model.", start.elapsed().as_millis());
    let input = "The easiest way of getting started is the [rustler Elixir library](https://hex.pm/packages/rustler).

- Add the [rustler Elixir library](https://hex.pm/packages/rustler) as a
  dependency of your project.
- Run `mix rustler.new` to generate a new NIF in your project. Follow the
  instructions.
- If you are already using [`serde`](https://serde.rs) and/or have been using
  `serde_rustler` before, please enable the `serde` feature in your NIF crate's
  `Cargo.toml` on the `rustler` dependency.
";
    let start = Instant::now();
    let output = model.predict(input);
    info!("Took {}ms to predict.", start.elapsed().as_millis());
    let mut tokens = output.unwrap();
    tokens.retain(filter_token);
    round_scores(&mut tokens);
    let text = Rope::from_str(input);
    let semantic_tokens = semantic_tokens(&text, &tokens, &Default::default());
    let combined = tokens.into_iter().zip(semantic_tokens).collect::<Vec<_>>();
    assert_debug_snapshot!(combined);
}

const PRECISION: f64 = 1e-3;

fn round_scores(predictions: &mut [POSToken]) {
    predictions
        .iter_mut()
        .for_each(|token| token.score = (token.score / PRECISION).round() * PRECISION)
}

fn init_tracing() {
    _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .from_env_lossy(),
        )
        .try_init();
}
