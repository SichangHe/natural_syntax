use std::str::FromStr;

use rust_bert::{
    pipelines::{
        pos_tagging::POSConfig,
        token_classification::{Token, TokenClassificationModel},
    },
    RustBertError,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Part-of-speech tagging model.
pub struct POSModel {
    pub model: TokenClassificationModel,
}

/// Iterator over the result of a POSModel prediction.
/// Implements `Iter<Item = Result<POSToken, PartOfSpeechError>>`.
pub type POSTokenResultIter = std::iter::Map<
    std::vec::IntoIter<Token>,
    fn(Token) -> Result<POSToken, <POSToken as TryFrom<Token>>::Error>,
>;

impl POSModel {
    pub fn try_default() -> Result<Self, RustBertError> {
        let model = TokenClassificationModel::new(POSConfig::default().into())?;
        Ok(Self { model })
    }

    /// Predict [`POSToken`]s for `input`.
    pub fn predict(&self, input: &str) -> POSTokenResultIter {
        let mut token_vecs = self.model.predict(&[input], true, false);
        debug_assert_eq!(1, token_vecs.len());
        token_vecs
            .pop()
            .unwrap()
            .into_iter()
            .map(POSToken::try_from)
    }
}

/// Parsed Token generated by a `TokenClassificationModel`
/// Adapted from `rust-bert`'s `Token` struct.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct POSToken {
    /// String representation of the Token
    pub word: String,
    /// Confidence score
    pub score: f64,
    /// Part-of-speech tag
    pub tag: PartOfSpeech,
    /// Label index
    pub label_index: i64,
    /// Token position index
    pub index: u16,
    /// Token word position index
    pub word_index: u16,
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

impl TryFrom<Token> for POSToken {
    type Error = PartOfSpeechError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        let Token {
            text,
            score,
            label,
            label_index,
            sentence: _inaccurate_so_useless,
            index,
            word_index,
            offset,
            mask: _we_do_not_care,
        } = value;
        match offset {
            Some(offset) => Ok(Self {
                word: text,
                score,
                tag: label.parse()?,
                label_index,
                index,
                word_index,
                offset_begin: offset.begin,
                offset_end: offset.end,
            }),
            None => Err(PartOfSpeechError::MissingOffset(text)),
        }
    }
}

/// Enum representing part-of-speech labels of MobileBERT, from
/// <https://huggingface.co/mrm8488/mobilebert-finetuned-pos/resolve/main/config.json>.
// NOTE: ChatGPT generated the docstrings, so they may be inaccurate.
#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum PartOfSpeech {
    /// Coordinating conjunction
    CC,
    /// Cardinal number
    CD,
    /// Determiner
    DT,
    /// Existential there
    EX,
    /// Foreign word
    FW,
    /// Preposition or subordinating conjunction
    IN,
    /// Adjective
    JJ,
    /// Adjective, comparative
    JJR,
    /// Adjective, superlative
    JJS,
    /// Modal
    MD,
    /// Noun, singular or mass
    NN,
    /// Proper noun, singular
    NNP,
    /// Proper noun, plural
    NNPS,
    /// Noun, plural
    NNS,
    /// Other (not a part of speech)
    O,
    /// Predeterminer
    PDT,
    /// Possessive ending
    POS,
    /// Personal pronoun
    PRP,
    /// Adverb
    RB,
    /// Adverb, comparative
    RBR,
    /// Adverb, superlative
    RBS,
    /// Particle
    RP,
    /// Symbol
    SYM,
    /// to
    TO,
    /// Interjection
    UH,
    /// Verb, base form
    VB,
    /// Verb, past tense
    VBD,
    /// Verb, gerund or present participle
    VBG,
    /// Verb, past participle
    VBN,
    /// Verb, non-3rd person singular present
    VBP,
    /// Verb, 3rd person singular present
    VBZ,
    /// Wh-determiner
    WDT,
    /// Wh-pronoun
    WP,
    /// Wh-adverb
    WRB,
}

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
    #[error("Token `{0}` without offset is ignored")]
    MissingOffset(String),
}

#[cfg(test)]
mod tests;
