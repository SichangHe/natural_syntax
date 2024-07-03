use std::str::FromStr;

use rust_bert::pipelines::pos_tagging::POSTag;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Tagged {
    pub word: String,
    pub score: f64,
    pub tag: PartOfSpeech,
}

impl Tagged {
    pub fn tag_with_confidence(&self, confidence: f64) -> Option<PartOfSpeech> {
        (self.score >= confidence).then_some(self.tag)
    }
}

impl TryFrom<POSTag> for Tagged {
    type Error = PartOfSpeechError;

    fn try_from(value: POSTag) -> Result<Self, Self::Error> {
        Ok(Self {
            word: value.word,
            score: value.score,
            tag: value.label.parse()?,
        })
    }
}

/// Enum representing part-of-speech labels of MobileBERT, from
/// <https://huggingface.co/mrm8488/mobilebert-finetuned-pos/resolve/main/config.json>.
// NOTE: ChatGPT generated the docstrings, so they may be inaccurate.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    /// A punctuation, as determined by RustBERT after the model's output.
    Punctuation,
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
            "." => Ok(Self::Punctuation),
            _ => Err(PartOfSpeechError(input.into())),
        }
    }
}

#[derive(Clone, Debug, Error)]
#[error("Unknown part of speech label `{0}`")]
pub struct PartOfSpeechError(String);

#[cfg(test)]
mod tests;
