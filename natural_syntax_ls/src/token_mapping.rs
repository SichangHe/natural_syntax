use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenMap(HashMap<PartOfSpeech, Option<TokenBits>>);

impl TokenMap {
    pub fn extend(&mut self, update: HashMap<PartOfSpeech, Option<TokenTypeNModifiers>>) {
        let extension = update.into_iter().map(|(pos, option)| {
            let token_bits = option.map(
                |TokenTypeNModifiers {
                     r#type,
                     modifiers: token_modifiers,
                 }| TokenBits {
                    token_type: r#type as u32,
                    token_modifiers_bitset: modifiers_to_bitmap(&token_modifiers),
                },
            );
            (pos, token_bits)
        });
        self.0.extend(extension);
    }

    pub fn get(&self, pos: PartOfSpeech) -> Option<TokenBits> {
        match self.0.get(&pos) {
            Some(&Some(token_type_n_modifier)) => Some(token_type_n_modifier),
            _ => None,
        }
    }
}

impl Default for TokenMap {
    fn default() -> Self {
        Self(
            (0..N_PART_OF_SPEECH)
                .map(|index| {
                    let pos = PartOfSpeech::from_u8(index).unwrap();
                    (pos, Some(pos2token_bits(pos)))
                })
                .collect(),
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct TokenTypeNModifiers {
    pub r#type: TokenType,
    #[serde(default)]
    pub modifiers: Vec<TokenModifier>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOKEN_MAP_JSON: &str = r#"{"CC":{"type":"modifier","modifiers":["readonly"]},"TO":{"type":"enumMember"},"IN":null}"#;

    #[test]
    fn extend_default() {
        let mut expected = TokenMap::default();
        let mut actual = expected.clone();
        expected.0.extend([
            (
                PartOfSpeech::CC,
                Some(TokenBits {
                    token_type: TokenType::Modifier as u32,
                    token_modifiers_bitset: modifiers_to_bitmap(&[TokenModifier::Readonly]),
                }),
            ),
            (
                PartOfSpeech::TO,
                Some(TokenBits {
                    token_type: TokenType::EnumMember as u32,
                    token_modifiers_bitset: 0,
                }),
            ),
            (PartOfSpeech::IN, None),
        ]);
        let update = serde_json::from_str(TOKEN_MAP_JSON).unwrap();
        actual.extend(update);
        assert_eq!(expected, actual);
    }
}
