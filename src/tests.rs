use super::*;

use std::time::Instant;

use insta::assert_debug_snapshot;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

#[test]
fn paragraph_pos() {
    init_tracing();
    let start = Instant::now();
    let model = POSModel::try_default().unwrap();
    info!("Took {}ms to load the model.", start.elapsed().as_millis());
    let input = "Extracts Part of Speech tags (Noun, Verb, Adjective…) from text. A lightweight pretrained model using MobileBERT is available for English.";
    let start = Instant::now();
    let output = model.predict(input);
    info!("Took {}ms to predict.", start.elapsed().as_millis());
    let mut parsed = output.unwrap();
    round_scores(&mut parsed);
    assert_debug_snapshot!(parsed);
}

#[test]
fn markdown_pos() {
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
    let mut parsed = output.unwrap();
    round_scores(&mut parsed);
    assert_debug_snapshot!(parsed);
}

const PRECISION: f64 = 1e-3;

#[test]
fn empty_unicode_and_long_text() -> Result<(), POSModelError> {
    let model = POSModel::try_default()?;
    assert!(model.predict("")?.is_empty());
    let input = format!(
        "😀 Café naïve birds fly.\r\n{}",
        "The birds fly. ".repeat(140)
    );
    let chars: Vec<_> = input.chars().collect();
    let tokens = model.predict(&input)?;
    assert!(tokens.len() > MAX_N_TOKEN);
    let mut end = 0;
    for token in &tokens {
        assert!(token.offset_begin >= end);
        assert!(token.offset_end > token.offset_begin);
        assert_eq!(
            chars[token.offset_begin as usize..token.offset_end as usize]
                .iter()
                .collect::<String>(),
            token.word
        );
        assert!((0.0..=1.0).contains(&token.score));
        end = token.offset_end;
    }
    assert_eq!(tokens.last().map(|token| token.word.as_str()), Some("."));
    Ok(())
}

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
