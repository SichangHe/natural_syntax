// NOTE: Enum numeric values correspond to their indexes in
// `SemanticTokensLegend`, and need to be incremental.
use super::*;

#[derive(
    Copy, Clone, Debug, Eq, FromPrimitive, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum TokenType {
    Namespace = 0,
    Type = 1,
    Class = 2,
    Enum = 3,
    Interface = 4,
    Struct = 5,
    TypeParameter = 6,
    Parameter = 7,
    Variable = 8,
    Property = 9,
    EnumMember = 10,
    Event = 11,
    Function = 12,
    Method = 13,
    Macro = 14,
    Keyword = 15,
    Modifier = 16,
    Comment = 17,
    String = 18,
    Number = 19,
    Regexp = 20,
    Operator = 21,
    Decorator = 22,
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
            TokenType::Namespace => SemanticTokenType::NAMESPACE,
            TokenType::Type => SemanticTokenType::TYPE,
            TokenType::Class => SemanticTokenType::CLASS,
            TokenType::Enum => SemanticTokenType::ENUM,
            TokenType::Interface => SemanticTokenType::INTERFACE,
            TokenType::Struct => SemanticTokenType::STRUCT,
            TokenType::TypeParameter => SemanticTokenType::TYPE_PARAMETER,
            TokenType::Parameter => SemanticTokenType::PARAMETER,
            TokenType::Variable => SemanticTokenType::VARIABLE,
            TokenType::Property => SemanticTokenType::PROPERTY,
            TokenType::EnumMember => SemanticTokenType::ENUM_MEMBER,
            TokenType::Event => SemanticTokenType::EVENT,
            TokenType::Function => SemanticTokenType::FUNCTION,
            TokenType::Method => SemanticTokenType::METHOD,
            TokenType::Macro => SemanticTokenType::MACRO,
            TokenType::Keyword => SemanticTokenType::KEYWORD,
            TokenType::Modifier => SemanticTokenType::MODIFIER,
            TokenType::Comment => SemanticTokenType::COMMENT,
            TokenType::String => SemanticTokenType::STRING,
            TokenType::Number => SemanticTokenType::NUMBER,
            TokenType::Regexp => SemanticTokenType::REGEXP,
            TokenType::Operator => SemanticTokenType::OPERATOR,
            TokenType::Decorator => SemanticTokenType::DECORATOR,
        }
    }
}

#[derive(
    Copy, Clone, Debug, Eq, FromPrimitive, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize,
)]
#[serde(rename_all = "camelCase")]
pub enum TokenModifier {
    Declaration = 0,
    Definition = 1,
    Readonly = 2,
    Static = 3,
    Deprecated = 4,
    Abstract = 5,
    Async = 6,
    Modification = 7,
    Documentation = 8,
    DefaultLibrary = 9,
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
            TokenModifier::Declaration => SemanticTokenModifier::DECLARATION,
            TokenModifier::Definition => SemanticTokenModifier::DEFINITION,
            TokenModifier::Readonly => SemanticTokenModifier::READONLY,
            TokenModifier::Static => SemanticTokenModifier::STATIC,
            TokenModifier::Deprecated => SemanticTokenModifier::DEPRECATED,
            TokenModifier::Abstract => SemanticTokenModifier::ABSTRACT,
            TokenModifier::Async => SemanticTokenModifier::ASYNC,
            TokenModifier::Modification => SemanticTokenModifier::MODIFICATION,
            TokenModifier::Documentation => SemanticTokenModifier::DOCUMENTATION,
            TokenModifier::DefaultLibrary => SemanticTokenModifier::DEFAULT_LIBRARY,
        }
    }
}

/// The default mapping from part of speech to semantic token types and
/// modifiers.
pub const fn pos2token_bits(pos: PartOfSpeech) -> TokenBits {
    match pos {
        PartOfSpeech::CC => {
            // Coordinating conjunctions
            TokenBits {
                token_type: TokenType::Keyword as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[]),
            }
        }
        PartOfSpeech::CD => {
            // Cardinal numbers
            TokenBits {
                token_type: TokenType::Number as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[]),
            }
        }
        PartOfSpeech::DT => {
            // Determiners
            TokenBits {
                token_type: TokenType::String as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Documentation]),
            }
        }
        PartOfSpeech::EX => {
            // Existential "there"
            TokenBits {
                token_type: TokenType::Keyword as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Definition]),
            }
        }
        PartOfSpeech::FW => {
            // Foreign words
            TokenBits {
                token_type: TokenType::String as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[]),
            }
        }
        PartOfSpeech::IN => {
            // Prepositions
            TokenBits {
                token_type: TokenType::Comment as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Async]),
            }
        }
        PartOfSpeech::JJ => {
            // Adjectives
            TokenBits {
                token_type: TokenType::Type as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[]),
            }
        }
        PartOfSpeech::JJR => {
            // Comparative adjectives
            TokenBits {
                token_type: TokenType::Struct as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Modification]),
            }
        }
        PartOfSpeech::JJS => {
            // Superlative adjectives
            TokenBits {
                token_type: TokenType::Interface as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DefaultLibrary]),
            }
        }
        PartOfSpeech::MD => {
            // Modals
            TokenBits {
                token_type: TokenType::Keyword as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Readonly]),
            }
        }
        PartOfSpeech::NN => {
            // Nouns (singular or mass)
            TokenBits {
                token_type: TokenType::Parameter as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[]),
            }
        }
        PartOfSpeech::NNP => {
            // Proper nouns (singular)
            TokenBits {
                token_type: TokenType::Parameter as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Declaration]),
            }
        }
        PartOfSpeech::NNPS => {
            // Proper nouns (plural)
            TokenBits {
                token_type: TokenType::Parameter as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[
                    TokenModifier::Declaration,
                    TokenModifier::Modification,
                ]),
            }
        }
        PartOfSpeech::NNS => {
            // Nouns (plural)
            TokenBits {
                token_type: TokenType::Parameter as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Modification]),
            }
        }
        PartOfSpeech::O => {
            // Other (not a part of speech)
            TokenBits {
                token_type: TokenType::Comment as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Deprecated]),
            }
        }
        PartOfSpeech::PDT => {
            // Predeterminers
            TokenBits {
                token_type: TokenType::String as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Abstract]),
            }
        }
        PartOfSpeech::POS => {
            // Possessive endings
            TokenBits {
                token_type: TokenType::Property as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Declaration]),
            }
        }
        PartOfSpeech::PRP => {
            // Personal pronouns
            TokenBits {
                token_type: TokenType::Property as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[]),
            }
        }
        PartOfSpeech::RB => {
            // Adverbs
            TokenBits {
                token_type: TokenType::EnumMember as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[]),
            }
        }
        PartOfSpeech::RBR => {
            // Comparative adverbs
            TokenBits {
                token_type: TokenType::EnumMember as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Async]),
            }
        }
        PartOfSpeech::RBS => {
            // Superlative adverbs
            TokenBits {
                token_type: TokenType::EnumMember as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DefaultLibrary]),
            }
        }
        PartOfSpeech::RP => {
            // Particles
            TokenBits {
                token_type: TokenType::Operator as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[]),
            }
        }
        PartOfSpeech::SYM => {
            // Symbols
            TokenBits {
                token_type: TokenType::Operator as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Documentation]),
            }
        }
        PartOfSpeech::TO => {
            // "To"
            TokenBits {
                token_type: TokenType::Keyword as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Static]),
            }
        }
        PartOfSpeech::UH => {
            // Interjections
            TokenBits {
                token_type: TokenType::Keyword as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Modification]),
            }
        }
        PartOfSpeech::VB => {
            // Base form verbs
            TokenBits {
                token_type: TokenType::Function as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[]),
            }
        }
        PartOfSpeech::VBD => {
            // Past tense verbs
            TokenBits {
                token_type: TokenType::Function as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Modification]),
            }
        }
        PartOfSpeech::VBG => {
            // Gerund or present participle verbs
            TokenBits {
                token_type: TokenType::Function as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Async]),
            }
        }
        PartOfSpeech::VBN => {
            // Past participle verbs
            TokenBits {
                token_type: TokenType::Method as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DefaultLibrary]),
            }
        }
        PartOfSpeech::VBP => {
            // Non-3rd person singular present verbs
            TokenBits {
                token_type: TokenType::Function as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Readonly]),
            }
        }
        PartOfSpeech::VBZ => {
            // 3rd person singular present verbs
            TokenBits {
                token_type: TokenType::Method as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Static]),
            }
        }
        PartOfSpeech::WDT => {
            // Wh-determiners
            TokenBits {
                token_type: TokenType::Keyword as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Documentation]),
            }
        }
        PartOfSpeech::WP => {
            // Wh-pronouns
            TokenBits {
                token_type: TokenType::Keyword as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::DefaultLibrary]),
            }
        }
        PartOfSpeech::WRB => {
            // Wh-adverbs
            TokenBits {
                token_type: TokenType::Keyword as u32,
                token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Async]),
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TokenBits {
    pub token_type: u32,
    pub token_modifiers_bitset: u32,
}
