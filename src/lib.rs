use std::{iter::zip, str::FromStr};

use hf_hub::{
    api::sync::{ApiBuilder, ApiError},
    Repo, RepoType,
};
#[cfg(feature = "num")]
use num_derive::{FromPrimitive, ToPrimitive};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokenizers::{Encoding, Tokenizer};
use tract_onnx::prelude::*;

/// ONNX export of MobileBERT fine-tuned for part-of-speech tagging,
/// downloaded from Hugging Face on first use.
pub const MODEL_REPO: &str = "onnx-community/mobilebert-finetuned-pos-ONNX";
/// Pinned so that [`LABELS`] and the snapshots stay valid.
pub const MODEL_REVISION: &str = "5b7653c4e2078ca6e6a7fd099c1f558eb066315b";
const MODEL_FILE: &str = "onnx/model.onnx";
/// `id2label` of the model, from
/// <https://huggingface.co/onnx-community/mobilebert-finetuned-pos-ONNX/blob/main/config.json>.
const LABELS: [PartOfSpeech; N_PART_OF_SPEECH as usize] = {
    use PartOfSpeech::*;
    [
        O, CC, CD, DT, EX, FW, IN, JJ, JJR, JJS, MD, NN, NNP, NNPS, NNS, PDT, POS, PRP, RB, RBR,
        RBS, RP, SYM, TO, UH, VB, VBD, VBG, VBN, VBP, VBZ, WDT, WP, WRB,
    ]
};
/// `max_position_embeddings` of the model.
const MAX_N_TOKEN: usize = 512;
const CLS_ID: i64 = 101;
const SEP_ID: i64 = 102;

/// Part-of-speech tagging model.
// 🧑 “fix natural_syntax_ls. It’s currently hard to install.”
pub struct POSModel {
    tokenizer: Tokenizer,
    model: Arc<TypedRunnableModel>,
}

impl POSModel {
    pub fn try_default() -> Result<Self, POSModelError> {
        let repo = ApiBuilder::from_env()
            .with_progress(false)
            .build()?
            .repo(Repo::with_revision(
                MODEL_REPO.into(),
                RepoType::Model,
                MODEL_REVISION.into(),
            ));
        let tokenizer = Tokenizer::from_file(repo.get("tokenizer.json")?)?;
        let model = tract_onnx::onnx()
            .model_for_path(repo.get(MODEL_FILE)?)?
            .into_optimized()?
            .into_runnable()?;
        Ok(Self { tokenizer, model })
    }

    /// Predict [`POSToken`]s for `input`, one per word.
    pub fn predict(&self, input: &str) -> Result<Vec<POSToken>, POSModelError> {
        let encoding = self.tokenizer.encode(input, false)?;
        let mut predictions = Vec::with_capacity(encoding.len());
        for ids in encoding.get_ids().chunks(MAX_N_TOKEN - 2) {
            predictions.extend(self.classify(ids)?);
        }
        Ok(words(input, &encoding, &predictions))
    }

    /// Most likely part of speech and its confidence score for
    /// each of the token `ids`.
    fn classify(&self, ids: &[u32]) -> TractResult<Vec<(PartOfSpeech, f64)>> {
        let n_token = ids.len() + 2;
        let ids = [CLS_ID]
            .into_iter()
            .chain(ids.iter().map(|&id| id as i64))
            .chain([SEP_ID])
            .collect::<Vec<_>>();
        let inputs = tvec![
            tensor1(&ids),
            tensor1(&vec![1i64; n_token]),
            tensor1(&vec![0i64; n_token]),
        ]
        .into_iter()
        .map(|tensor| Ok(tensor.into_shape(&[1, n_token])?.into()))
        .collect::<TractResult<TVec<TValue>>>()?;
        let outputs = self.model.run(inputs)?;
        let logits = outputs[0].to_plain_array_view::<f32>()?;
        Ok(logits
            .rows()
            .into_iter()
            .skip(1)
            .take(n_token - 2)
            .map(|row| {
                let (i_max, max) = zip(0.., row)
                    .max_by(|(_, a), (_, b)| a.total_cmp(b))
                    .expect("The model outputs one logit per label.");
                let sum = row.iter().map(|logit| (logit - max).exp()).sum::<f32>();
                (LABELS[i_max], 1. / sum as f64)
            })
            .collect())
    }
}

/// Merge the sub-word tokens in `encoding` of `input` into words,
/// each tagged with the prediction of its first token.
fn words(input: &str, encoding: &Encoding, predictions: &[(PartOfSpeech, f64)]) -> Vec<POSToken> {
    let mut byte_words: Vec<(usize, usize, PartOfSpeech, f64)> = Vec::new();
    let mut prev_word_id = None;
    for ((word_id, &(begin, end)), &(tag, score)) in zip(
        zip(encoding.get_word_ids(), encoding.get_offsets()),
        predictions,
    ) {
        match byte_words.last_mut() {
            Some(word) if *word_id == prev_word_id => word.1 = end,
            _ => byte_words.push((begin, end, tag, score)),
        }
        prev_word_id = *word_id;
    }
    let (mut i_byte, mut i_char) = (0, 0);
    byte_words
        .into_iter()
        .map(|(begin, end, tag, score)| {
            let word = &input[begin..end];
            i_char += input[i_byte..begin].chars().count() as u32;
            i_byte = begin;
            POSToken {
                word: word.into(),
                score,
                tag,
                offset_begin: i_char,
                offset_end: i_char + word.chars().count() as u32,
            }
        })
        .collect()
}

#[derive(Debug, Error)]
pub enum POSModelError {
    #[error("Downloading the model: {0}")]
    Download(#[from] ApiError),
    #[error("Tokenizing: {0}")]
    Tokenizer(#[from] tokenizers::Error),
    #[error("Running the model: {0}")]
    Model(#[from] TractError),
}

/// A word tagged with its part of speech.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct POSToken {
    /// String representation of the Token
    pub word: String,
    /// Confidence score
    pub score: f64,
    /// Part-of-speech tag
    pub tag: PartOfSpeech,
    /// Token offset beginning (in unicode points) relative to the input string
    pub offset_begin: u32,
    /// Token offset end (in unicode points) relative to the input string
    pub offset_end: u32,
}

impl POSToken {
    pub fn tag_with_confidence(&self, confidence: f64) -> Option<PartOfSpeech> {
        (self.score > confidence).then_some(self.tag)
    }
}

/// Enum representing part-of-speech labels of MobileBERT, from
/// <https://huggingface.co/mrm8488/mobilebert-finetuned-pos/resolve/main/config.json>.
// NOTE: ChatGPT generated the docstrings, so they may be inaccurate.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "num", derive(FromPrimitive, ToPrimitive))]
pub enum PartOfSpeech {
    /// Coordinating conjunction
    CC = 0,
    /// Cardinal number
    CD = 1,
    /// Determiner
    DT = 2,
    /// Existential there
    EX = 3,
    /// Foreign word
    FW = 4,
    /// Preposition or subordinating conjunction
    IN = 5,
    /// Adjective
    JJ = 6,
    /// Adjective, comparative
    JJR = 7,
    /// Adjective, superlative
    JJS = 8,
    /// Modal
    MD = 9,
    /// Noun, singular or mass
    NN = 10,
    /// Proper noun, singular
    NNP = 11,
    /// Proper noun, plural
    NNPS = 12,
    /// Noun, plural
    NNS = 13,
    /// Other (not a part of speech)
    #[default]
    O = 14,
    /// Predeterminer
    PDT = 15,
    /// Possessive ending
    POS = 16,
    /// Personal pronoun
    PRP = 17,
    /// Adverb
    RB = 18,
    /// Adverb, comparative
    RBR = 19,
    /// Adverb, superlative
    RBS = 20,
    /// Particle
    RP = 21,
    /// Symbol
    SYM = 22,
    /// to
    TO = 23,
    /// Interjection
    UH = 24,
    /// Verb, base form
    VB = 25,
    /// Verb, past tense
    VBD = 26,
    /// Verb, gerund or present participle
    VBG = 27,
    /// Verb, past participle
    VBN = 28,
    /// Verb, non-3rd person singular present
    VBP = 29,
    /// Verb, 3rd person singular present
    VBZ = 30,
    /// Wh-determiner
    WDT = 31,
    /// Wh-pronoun
    WP = 32,
    /// Wh-adverb
    WRB = 33,
}

pub const N_PART_OF_SPEECH: u8 = 34;

impl FromStr for PartOfSpeech {
    type Err = PartOfSpeechError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "CC" => Ok(Self::CC),
            "CD" => Ok(Self::CD),
            "DT" => Ok(Self::DT),
            "EX" => Ok(Self::EX),
            "FW" => Ok(Self::FW),
            "IN" => Ok(Self::IN),
            "JJ" => Ok(Self::JJ),
            "JJR" => Ok(Self::JJR),
            "JJS" => Ok(Self::JJS),
            "MD" => Ok(Self::MD),
            "NN" => Ok(Self::NN),
            "NNP" => Ok(Self::NNP),
            "NNPS" => Ok(Self::NNPS),
            "NNS" => Ok(Self::NNS),
            "O" => Ok(Self::O),
            "PDT" => Ok(Self::PDT),
            "POS" => Ok(Self::POS),
            "PRP" => Ok(Self::PRP),
            "RB" => Ok(Self::RB),
            "RBR" => Ok(Self::RBR),
            "RBS" => Ok(Self::RBS),
            "RP" => Ok(Self::RP),
            "SYM" => Ok(Self::SYM),
            "TO" => Ok(Self::TO),
            "UH" => Ok(Self::UH),
            "VB" => Ok(Self::VB),
            "VBD" => Ok(Self::VBD),
            "VBG" => Ok(Self::VBG),
            "VBN" => Ok(Self::VBN),
            "VBP" => Ok(Self::VBP),
            "VBZ" => Ok(Self::VBZ),
            "WDT" => Ok(Self::WDT),
            "WP" => Ok(Self::WP),
            "WRB" => Ok(Self::WRB),
            _ => Err(PartOfSpeechError::UnknownLabel(input.into())),
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum PartOfSpeechError {
    #[error("Unknown part of speech label `{0}`")]
    UnknownLabel(String),
}

#[cfg(test)]
mod tests;
