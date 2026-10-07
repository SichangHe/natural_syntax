/** Semantic token legend; a token type is its index in `TOKEN_TYPES`. */
export const TOKEN_TYPES = ['namespace', 'type', 'class', 'enum', 'interface', 'struct', 'typeParameter',
    'parameter', 'variable', 'property', 'enumMember', 'event', 'function', 'method', 'macro', 'keyword',
    'modifier', 'comment', 'string', 'number', 'regexp', 'operator', 'decorator'];
/** A token modifier is the bit at its index in `TOKEN_MODIFIERS`. */
export const TOKEN_MODIFIERS = ['declaration', 'definition', 'readonly', 'static', 'deprecated', 'abstract',
    'async', 'modification', 'documentation', 'defaultLibrary'];
/** Part of speech tag → `{type, modifiers}`, same as `pos2token_bits` of the Rust server. */
export const DEFAULT_TOKEN_MAP = {
    CC: { type: 'keyword' },
    CD: { type: 'number' },
    DT: { type: 'string', modifiers: ['documentation'] },
    EX: { type: 'keyword', modifiers: ['definition'] },
    FW: { type: 'string' },
    IN: { type: 'comment', modifiers: ['async'] },
    JJ: { type: 'type' },
    JJR: { type: 'struct', modifiers: ['modification'] },
    JJS: { type: 'interface', modifiers: ['defaultLibrary'] },
    MD: { type: 'keyword', modifiers: ['readonly'] },
    NN: { type: 'parameter' },
    NNP: { type: 'parameter', modifiers: ['declaration'] },
    NNPS: { type: 'parameter', modifiers: ['declaration', 'modification'] },
    NNS: { type: 'parameter', modifiers: ['modification'] },
    O: { type: 'comment', modifiers: ['deprecated'] },
    PDT: { type: 'string', modifiers: ['abstract'] },
    POS: { type: 'property', modifiers: ['declaration'] },
    PRP: { type: 'property' },
    RB: { type: 'enumMember' },
    RBR: { type: 'enumMember', modifiers: ['async'] },
    RBS: { type: 'enumMember', modifiers: ['defaultLibrary'] },
    RP: { type: 'operator' },
    SYM: { type: 'operator', modifiers: ['documentation'] },
    TO: { type: 'keyword', modifiers: ['static'] },
    UH: { type: 'keyword', modifiers: ['modification'] },
    VB: { type: 'function' },
    VBD: { type: 'function', modifiers: ['modification'] },
    VBG: { type: 'function', modifiers: ['async'] },
    VBN: { type: 'method', modifiers: ['defaultLibrary'] },
    VBP: { type: 'function', modifiers: ['readonly'] },
    VBZ: { type: 'method', modifiers: ['static'] },
    WDT: { type: 'keyword', modifiers: ['documentation'] },
    WP: { type: 'keyword', modifiers: ['defaultLibrary'] },
    WRB: { type: 'keyword', modifiers: ['async'] },
};
const TOKEN_SCORE_THRESHOLD = 1 / 3;

/**
 * `DEFAULT_TOKEN_MAP` with `update` applied, as tag → `[type index, modifier bits]`;
 * a `null` in `update` disables the tag. Returns `{ok: true, value}` or `{ok: false, error}`.
 */
export function tokenMap(update = {}) {
    if (update === null || typeof update !== 'object') return { ok: false, error: '`token_map_update` must be an object.' };
    const value = {};
    for (const [tag, entry] of Object.entries({ ...DEFAULT_TOKEN_MAP, ...update })) {
        if (!(tag in DEFAULT_TOKEN_MAP)) return { ok: false, error: `Unknown part of speech \`${tag}\`.` };
        if (entry === null) continue;
        const type = TOKEN_TYPES.indexOf(entry?.type);
        const modifiers = (entry?.modifiers ?? []).map(modifier => TOKEN_MODIFIERS.indexOf(modifier));
        if (type < 0 || modifiers.includes(-1)) return { ok: false, error: `Invalid token for \`${tag}\`: ${JSON.stringify(entry)}.` };
        value[tag] = [type, modifiers.reduce((bits, index) => bits | 1 << index, 0)];
    }
    return { ok: true, value };
}

/**
 * LSP semantic token data for `words` predicted from `text`, positions in UTF-16 units.
 * Drop low-score, punctuation-only, and unmapped words.
 */
export function semanticTokens(text, words, map) {
    const kept = words.filter(({ word, score, tag }) => score > TOKEN_SCORE_THRESHOLD && map[tag]
        && /[^!-\/:-@\[-`{-~]/.test(word));
    const data = [];
    let [i_word, i_code_point, i_unit, line, column, prev_line, prev_column] = [0, 0, 0, 0, 0, 0, 0];
    for (const char of text) {
        if (i_word >= kept.length) break;
        const { word, tag, offset_begin } = kept[i_word];
        if (i_code_point === offset_begin) {
            data.push(line - prev_line, line === prev_line ? column - prev_column : column, word.length, ...map[tag]);
            [prev_line, prev_column] = [line, column];
            i_word++;
        }
        i_code_point++;
        i_unit += char.length;
        // The `\r` of `\r\n` does not end the line; its `\n` does.
        if (char === '\n' || (char === '\r' && text[i_unit] !== '\n')) [line, column] = [line + 1, 0];
        else column += char.length;
    }
    return data;
}
