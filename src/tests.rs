use super::*;

use std::time::Instant;

use insta::assert_debug_snapshot;

#[test]
fn paragraph_pos() {
    let start = Instant::now();
    let model = POSModel::try_default().unwrap();
    println!("Took {}ms to load the model.", start.elapsed().as_millis());
    let input = "Extracts Part of Speech tags (Noun, Verb, Adjective…) from text. A lightweight pretrained model using MobileBERT is available for English.";
    let start = Instant::now();
    let output = model.predict(input);
    println!("Took {}ms to predict.", start.elapsed().as_millis());
    let parsed = output.collect::<Result<Vec<_>, _>>().unwrap();
    assert_debug_snapshot!(parsed);
}
