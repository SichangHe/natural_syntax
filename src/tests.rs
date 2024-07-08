use super::*;

use std::time::Instant;

use insta::assert_debug_snapshot;

#[test]
fn paragraph_pos() {
    let start = Instant::now();
    let model = pos_model().unwrap();
    println!("Took {}ms to load the model.", start.elapsed().as_millis());
    let input = ["Extracts Part of Speech tags (Noun, Verb, Adjective…) from text. A lightweight pretrained model using MobileBERT is available for English."];
    let start = Instant::now();
    let mut output = model.predict(&input, true, false);
    println!("Took {}ms to predict.", start.elapsed().as_millis());
    assert_eq!(1, output.len());
    let parsed = output[0]
        .drain(..)
        .map(POSToken::try_from)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_debug_snapshot!(parsed);
}
