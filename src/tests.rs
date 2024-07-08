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
    let mut parsed = output.collect::<Result<Vec<_>, _>>().unwrap();
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
    let mut parsed = output.collect::<Result<Vec<_>, _>>().unwrap();
    round_scores(&mut parsed);
    assert_debug_snapshot!(parsed);
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
