import { AutoTokenizer, BertForTokenClassification, Tensor } from '@huggingface/transformers';

export const MODEL_REPO = 'onnx-community/mobilebert-finetuned-pos-ONNX';
export const MODEL_REVISION = '5b7653c4e2078ca6e6a7fd099c1f558eb066315b';
const WORDS = /\[(?:MASK|CLS|SEP|UNK|PAD)\]|[\p{P}\p{Script=Han}\x21-\x2f\x3a-\x40\x5b-\x60\x7b-\x7e]|[^\s\p{P}\p{Script=Han}\x21-\x2f\x3a-\x40\x5b-\x60\x7b-\x7e]+/gu;

// 🧑 “And make a JS version of the idea.”
/** Load the pinned model. Returns `{ok: true, value}` or `{ok: false, error}`. */
export async function createPOSModel({ cache_dir, local_files_only = false } = {}) {
    try {
        const options = { revision: MODEL_REVISION, cache_dir, local_files_only };
        const tokenizer = await AutoTokenizer.from_pretrained(MODEL_REPO, options);
        const model = await BertForTokenClassification.from_pretrained(MODEL_REPO, {
            ...options, dtype: 'fp32', device: 'cpu',
        });
        return { ok: true, value: {
            /** Predict words with code-point offsets; retain the first subword's label. */
            async predict(input) {
                if (typeof input !== 'string') return { ok: false, error: 'Input must be a string.' };
                try {
                    const words = [];
                    const ids = [];
                    let prevEnd = 0;
                    let offset = 0;
                    for (const match of input.matchAll(WORDS)) {
                        offset += [...input.slice(prevEnd, match.index)].length;
                        const word = match[0];
                        const wordIds = tokenizer.encode(word, { add_special_tokens: false });
                        if (wordIds.length) words.push({ word, offset_begin: offset,
                            offset_end: offset + [...word].length, index: ids.length });
                        ids.push(...wordIds);
                        offset += [...word].length;
                        prevEnd = match.index + word.length;
                    }
                    const predictions = [];
                    for (let begin = 0; begin < ids.length; begin += 510) {
                        const chunk = [101, ...ids.slice(begin, begin + 510), 102];
                        const tensor = values => new Tensor('int64', BigInt64Array.from(values, BigInt), [1, values.length]);
                        const { logits } = await model({ input_ids: tensor(chunk),
                            attention_mask: tensor(chunk.map(() => 1)), token_type_ids: tensor(chunk.map(() => 0)) });
                        const nLabels = logits.dims[2];
                        for (let i = 1; i < chunk.length - 1; i++) {
                            const row = logits.data.subarray(i * nLabels, (i + 1) * nLabels);
                            const max = Math.max(...row);
                            const label = row.indexOf(max);
                            const score = 1 / row.reduce((sum, value) => sum + Math.exp(value - max), 0);
                            predictions.push({ tag: model.config.id2label[label], score });
                        }
                    }
                    return { ok: true, value: words.map(({ index, ...word }) => ({ ...word, ...predictions[index] })) };
                } catch (error) {
                    return { ok: false, error: String(error.message ?? error) };
                }
            },
            async dispose() { await model.dispose(); },
        } };
    } catch (error) {
        return { ok: false, error: String(error.message ?? error) };
    }
}
