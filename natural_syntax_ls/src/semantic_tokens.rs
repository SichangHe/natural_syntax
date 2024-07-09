// NOTE: Enum numeric values correspond to their indexes in
// `SemanticTokensLegend`, and need to be incremental.
use natural_syntax::PartOfSpeech;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use super::*;

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Eq, FromPrimitive, Hash, Ord, PartialEq, PartialOrd)]
pub enum TokenType {
    NAMESPACE = 0,
    TYPE = 1,
    CLASS = 2,
    ENUM = 3,
    INTERFACE = 4,
    STRUCT = 5,
    TYPE_PARAMETER = 6,
    PARAMETER = 7,
    VARIABLE = 8,
    PROPERTY = 9,
    ENUM_MEMBER = 10,
    EVENT = 11,
    FUNCTION = 12,
    METHOD = 13,
    MACRO = 14,
    KEYWORD = 15,
    MODIFIER = 16,
    COMMENT = 17,
    STRING = 18,
    NUMBER = 19,
    REGEXP = 20,
    OPERATOR = 21,
    DECORATOR = 22,
}

pub const N_TOKEN_TYPES: u8 = 23;

pub fn semantic_token_types() -> Vec<SemanticTokenType> {
    (0..N_TOKEN_TYPES)
        .map(|index| TokenType::from_u8(index).unwrap().into())
        .collect()
}

impl From<TokenType> for SemanticTokenType {
    fn from(val: TokenType) -> Self {
        match val {
            TokenType::NAMESPACE => SemanticTokenType::NAMESPACE,
            TokenType::TYPE => SemanticTokenType::TYPE,
            TokenType::CLASS => SemanticTokenType::CLASS,
            TokenType::ENUM => SemanticTokenType::ENUM,
            TokenType::INTERFACE => SemanticTokenType::INTERFACE,
            TokenType::STRUCT => SemanticTokenType::STRUCT,
            TokenType::TYPE_PARAMETER => SemanticTokenType::TYPE_PARAMETER,
            TokenType::PARAMETER => SemanticTokenType::PARAMETER,
            TokenType::VARIABLE => SemanticTokenType::VARIABLE,
            TokenType::PROPERTY => SemanticTokenType::PROPERTY,
            TokenType::ENUM_MEMBER => SemanticTokenType::ENUM_MEMBER,
            TokenType::EVENT => SemanticTokenType::EVENT,
            TokenType::FUNCTION => SemanticTokenType::FUNCTION,
            TokenType::METHOD => SemanticTokenType::METHOD,
            TokenType::MACRO => SemanticTokenType::MACRO,
            TokenType::KEYWORD => SemanticTokenType::KEYWORD,
            TokenType::MODIFIER => SemanticTokenType::MODIFIER,
            TokenType::COMMENT => SemanticTokenType::COMMENT,
            TokenType::STRING => SemanticTokenType::STRING,
            TokenType::NUMBER => SemanticTokenType::NUMBER,
            TokenType::REGEXP => SemanticTokenType::REGEXP,
            TokenType::OPERATOR => SemanticTokenType::OPERATOR,
            TokenType::DECORATOR => SemanticTokenType::DECORATOR,
        }
    }
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Eq, FromPrimitive, Hash, Ord, PartialEq, PartialOrd)]
pub enum TokenModifier {
    DECLARATION = 0,
    DEFINITION = 1,
    READONLY = 2,
    STATIC = 3,
    DEPRECATED = 4,
    ABSTRACT = 5,
    ASYNC = 6,
    MODIFICATION = 7,
    DOCUMENTATION = 8,
    DEFAULT_LIBRARY = 9,
}

pub const N_TOKEN_MODIFIERS: u8 = 10;

pub fn semantic_token_modifiers() -> Vec<SemanticTokenModifier> {
    (0..N_TOKEN_MODIFIERS)
        .map(|index| TokenModifier::from_u8(index).unwrap().into())
        .collect()
}

impl From<TokenModifier> for SemanticTokenModifier {
    fn from(val: TokenModifier) -> Self {
        match val {
            TokenModifier::DECLARATION => SemanticTokenModifier::DECLARATION,
            TokenModifier::DEFINITION => SemanticTokenModifier::DEFINITION,
            TokenModifier::READONLY => SemanticTokenModifier::READONLY,
            TokenModifier::STATIC => SemanticTokenModifier::STATIC,
            TokenModifier::DEPRECATED => SemanticTokenModifier::DEPRECATED,
            TokenModifier::ABSTRACT => SemanticTokenModifier::ABSTRACT,
            TokenModifier::ASYNC => SemanticTokenModifier::ASYNC,
            TokenModifier::MODIFICATION => SemanticTokenModifier::MODIFICATION,
            TokenModifier::DOCUMENTATION => SemanticTokenModifier::DOCUMENTATION,
            TokenModifier::DEFAULT_LIBRARY => SemanticTokenModifier::DEFAULT_LIBRARY,
        }
    }
}

// NOTE: This function is originally generated by ChatGPT;
// it might have made unreasonable assumptions.
pub const fn token_type_n_modifiers(pos: PartOfSpeech) -> TokenTypeNModifier {
    // Match the PartOfSpeech enum to TokenType and modifiers
    match pos {
        PartOfSpeech::CC => {
            // Coordinating conjunctions are classified as KEYWORD with no modifiers
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: 0,
            }
        }
        PartOfSpeech::CD => {
            // Cardinal numbers are classified as NUMBER with no modifiers
            TokenTypeNModifier {
                token_type: TokenType::NUMBER as u32,
                token_modifiers_bitset: 0,
            }
        }
        PartOfSpeech::DT => {
            // Determiners are classified as KEYWORD with DOCUMENTATION modifier
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DOCUMENTATION]),
            }
        }
        PartOfSpeech::EX => {
            // Existential "there" is classified as KEYWORD with DEPRECATED modifier
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DEPRECATED]),
            }
        }
        PartOfSpeech::FW => {
            // Foreign words are classified as STRING with no modifiers
            TokenTypeNModifier {
                token_type: TokenType::STRING as u32,
                token_modifiers_bitset: 0,
            }
        }
        PartOfSpeech::IN => {
            // Prepositions and subordinating conjunctions are classified as KEYWORD with ASYNC modifier
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::ASYNC]),
            }
        }
        PartOfSpeech::JJ => {
            // Adjectives are classified as TYPE with no modifiers
            TokenTypeNModifier {
                token_type: TokenType::TYPE as u32,
                token_modifiers_bitset: 0,
            }
        }
        PartOfSpeech::JJR => {
            // Comparative adjectives are classified as TYPE with MODIFICATION modifier
            TokenTypeNModifier {
                token_type: TokenType::TYPE as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::MODIFICATION]),
            }
        }
        PartOfSpeech::JJS => {
            // Superlative adjectives are classified as TYPE with DEFAULT_LIBRARY modifier
            TokenTypeNModifier {
                token_type: TokenType::TYPE as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DEFAULT_LIBRARY]),
            }
        }
        PartOfSpeech::MD => {
            // Modals are classified as KEYWORD with READONLY modifier
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::READONLY]),
            }
        }
        PartOfSpeech::NN => {
            // Nouns (singular or mass) are classified as TYPE with no modifiers
            TokenTypeNModifier {
                token_type: TokenType::TYPE as u32,
                token_modifiers_bitset: 0,
            }
        }
        PartOfSpeech::NNP => {
            // Proper nouns (singular) are classified as TYPE with DECLARATION modifier
            TokenTypeNModifier {
                token_type: TokenType::TYPE as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DECLARATION]),
            }
        }
        PartOfSpeech::NNPS => {
            // Proper nouns (plural) are classified as TYPE with DEFINITION modifier
            TokenTypeNModifier {
                token_type: TokenType::TYPE as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DEFINITION]),
            }
        }
        PartOfSpeech::NNS => {
            // Nouns (plural) are classified as TYPE with STATIC modifier
            TokenTypeNModifier {
                token_type: TokenType::TYPE as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::STATIC]),
            }
        }
        PartOfSpeech::O => {
            // Other (not a part of speech) is classified as COMMENT with no modifiers
            TokenTypeNModifier {
                token_type: TokenType::COMMENT as u32,
                token_modifiers_bitset: 0,
            }
        }
        PartOfSpeech::PDT => {
            // Predeterminers are classified as KEYWORD with ABSTRACT modifier
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::ABSTRACT]),
            }
        }
        PartOfSpeech::POS => {
            // Possessive endings are classified as KEYWORD with DEPRECATION modifier
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DEPRECATED]),
            }
        }
        PartOfSpeech::PRP => {
            // Personal pronouns are classified as TYPE_PARAMETER with no modifiers
            TokenTypeNModifier {
                token_type: TokenType::TYPE_PARAMETER as u32,
                token_modifiers_bitset: 0,
            }
        }
        PartOfSpeech::RB => {
            // Adverbs are classified as KEYWORD with MODIFIER modifier
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::MODIFICATION]),
            }
        }
        PartOfSpeech::RBR => {
            // Comparative adverbs are classified as KEYWORD with ASYNC modifier
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::ASYNC]),
            }
        }
        PartOfSpeech::RBS => {
            // Superlative adverbs are classified as KEYWORD with DEFAULT_LIBRARY modifier
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DEFAULT_LIBRARY]),
            }
        }
        PartOfSpeech::RP => {
            // Particles are classified as OPERATOR with no modifiers
            TokenTypeNModifier {
                token_type: TokenType::OPERATOR as u32,
                token_modifiers_bitset: 0,
            }
        }
        PartOfSpeech::SYM => {
            // Symbols are classified as OPERATOR with DOCUMENTATION modifier
            TokenTypeNModifier {
                token_type: TokenType::OPERATOR as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DOCUMENTATION]),
            }
        }
        PartOfSpeech::TO => {
            // "To" is classified as KEYWORD with STATIC modifier
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::STATIC]),
            }
        }
        PartOfSpeech::UH => {
            // Interjections are classified as KEYWORD with DEPRECATED modifier
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DEPRECATED]),
            }
        }
        PartOfSpeech::VB => {
            // Base form verbs are classified as FUNCTION with no modifiers
            TokenTypeNModifier {
                token_type: TokenType::FUNCTION as u32,
                token_modifiers_bitset: 0,
            }
        }
        PartOfSpeech::VBD => {
            // Past tense verbs are classified as FUNCTION with MODIFICATION modifier
            TokenTypeNModifier {
                token_type: TokenType::FUNCTION as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::MODIFICATION]),
            }
        }
        PartOfSpeech::VBG => {
            // Gerund or present participle verbs are classified as FUNCTION with ASYNC modifier
            TokenTypeNModifier {
                token_type: TokenType::FUNCTION as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::ASYNC]),
            }
        }
        PartOfSpeech::VBN => {
            // Past participle verbs are classified as FUNCTION with DEFAULT_LIBRARY modifier
            TokenTypeNModifier {
                token_type: TokenType::FUNCTION as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DEFAULT_LIBRARY]),
            }
        }
        PartOfSpeech::VBP => {
            // Non-3rd person singular present verbs are classified as FUNCTION with READONLY modifier
            TokenTypeNModifier {
                token_type: TokenType::FUNCTION as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::READONLY]),
            }
        }
        PartOfSpeech::VBZ => {
            // 3rd person singular present verbs are classified as FUNCTION with STATIC modifier
            TokenTypeNModifier {
                token_type: TokenType::FUNCTION as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::STATIC]),
            }
        }
        PartOfSpeech::WDT => {
            // Wh-determiners are classified as KEYWORD with DOCUMENTATION modifier
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DOCUMENTATION]),
            }
        }
        PartOfSpeech::WP => {
            // Wh-pronouns are classified as TYPE_PARAMETER with no modifiers
            TokenTypeNModifier {
                token_type: TokenType::TYPE_PARAMETER as u32,
                token_modifiers_bitset: 0,
            }
        }
        PartOfSpeech::WRB => {
            // Wh-adverbs are classified as KEYWORD with DEPRECATED modifier
            TokenTypeNModifier {
                token_type: TokenType::KEYWORD as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DEPRECATED]),
            }
        }
    }
}

/// Convert [`TokenModifier`]s to a bitmap as per the LSP spec at
/// <https://github.com/microsoft/vscode-extension-samples/blob/5ae1f7787122812dcc84e37427ca90af5ee09f14/semantic-tokens-sample/vscode.proposed.d.ts#L104>.
pub const fn modifiers_to_bitmap(modifiers: &[TokenModifier]) -> u32 {
    let mut bitmap = 0;
    let mut index = 0;
    while index < modifiers.len() {
        bitmap |= 1 << modifiers[index] as u32;
        index += 1;
    }
    bitmap
}

pub struct TokenTypeNModifier {
    pub token_type: u32,
    pub token_modifiers_bitset: u32,
}