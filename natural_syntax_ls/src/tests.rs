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

fn init_tracing() {
    _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .from_env_lossy(),
        )
        .try_init();
}
