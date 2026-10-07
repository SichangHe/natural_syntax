import assert from 'node:assert/strict';
import { test } from 'node:test';
import { spawnSync } from 'node:child_process';
import { createPOSModel } from './index.js';

test('real MobileBERT tags, Unicode offsets, and complete long documents', { timeout: 300_000 }, async () => {
    const loaded = await createPOSModel();
    assert.equal(loaded.ok, true, loaded.error);
    const model = loaded.value;
    try {
        const sentence = 'The quick brown fox jumps over the lazy dog.';
        const result = await model.predict(sentence);
        assert.equal(result.ok, true, result.error);
        assert.deepEqual(result.value.map(token => token.word), sentence.match(/\w+|\./g));
        assert.deepEqual(result.value.map(token => token.tag), ['DT', 'JJ', 'JJ', 'NN', 'NNS', 'IN', 'DT', 'JJ', 'NN', 'PDT']);
        for (const input of ['', ' \n\t', '😀 Café isn’t naïve. 中文', 'antidisestablishmentarianism', 'A [MASK] word.']) {
            const prediction = await model.predict(input);
            assert.equal(prediction.ok, true, prediction.error);
            const chars = [...input];
            for (const token of prediction.value) {
                assert.equal(chars.slice(token.offset_begin, token.offset_end).join(''), token.word);
                assert.ok(token.score > 0 && token.score <= 1);
                assert.equal(typeof token.tag, 'string');
            }
            if (!input.trim()) assert.deepEqual(prediction.value, []);
            if (input === 'antidisestablishmentarianism') assert.equal(prediction.value.length, 1);
            if (input.includes('[MASK]')) assert.deepEqual(prediction.value.map(token => token.word), ['A', '[MASK]', 'word', '.']);
        }
        const longInput = 'The dog runs. '.repeat(160);
        const long = await model.predict(longInput);
        assert.equal(long.ok, true, long.error);
        assert.equal(long.value.length, 640);
        assert.equal(long.value.at(-1).offset_end, [...longInput.trimEnd()].length);
        assert.equal((await model.predict(null)).ok, false);
    } finally { await model.dispose(); }
});

test('CLI accepts stdin and reports usage without loading the model', { timeout: 300_000 }, () => {
    const cwd = new URL('.', import.meta.url);
    const help = spawnSync(process.execPath, ['cli.js', '--help'], { cwd, encoding: 'utf8', timeout: 10_000 });
    assert.equal(help.status, 0, help.stderr);
    assert.match(help.stdout, /Usage:/);
    const tagged = spawnSync(process.execPath, ['cli.js'], { cwd, input: 'Dogs run.', encoding: 'utf8', timeout: 290_000 });
    assert.equal(tagged.status, 0, tagged.stderr);
    assert.deepEqual(JSON.parse(tagged.stdout).map(token => token.word), ['Dogs', 'run', '.']);
});
