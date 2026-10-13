import assert from 'node:assert/strict';
import { chromium } from 'playwright';
import { createServer } from 'vite';

const server = await createServer({ server: { host: '127.0.0.1', port: 0 } });
await server.listen();
let browser;
try {
    browser = await chromium.launch({ executablePath: process.env.CHROME_PATH, args: ['--no-sandbox'] });
    const page = await browser.newPage();
    page.on('console', message => { if (message.type() === 'error') console.error(message.text()); });
    await page.goto(`${server.resolvedUrls.local[0]}browser.html`);
    await page.waitForFunction(() => document.querySelector('#status').textContent !== 'Loading model…', null, { timeout: 300_000 });
    assert.equal(await page.locator('#status').textContent(), 'Ready');
    for (const text of ['The quick brown fox jumps over the lazy dog.', '😀 Café isn’t naïve. 中文', 'The dog runs. '.repeat(160)]) {
        await page.locator('#text').fill(text);
        await page.locator('button').click();
        await page.waitForFunction(() => document.querySelector('#status').textContent !== 'Tagging…', null, { timeout: 300_000 });
        assert.equal(await page.locator('#status').textContent(), 'Ready');
        const words = JSON.parse(await page.locator('#result').textContent());
        assert.ok(words.length > 0);
        for (const word of words) {
            assert.equal([...text].slice(word.offset_begin, word.offset_end).join(''), word.word);
            assert.ok(word.score > 0 && word.score <= 1);
        }
        if (text.startsWith('The quick')) assert.deepEqual(words.map(word => word.tag), ['DT', 'JJ', 'JJ', 'NN', 'NNS', 'IN', 'DT', 'JJ', 'NN', 'PDT']);
        if (text.startsWith('The dog')) assert.equal(words.length, 640);
    }
} finally {
    await browser?.close();
    await server.close();
}
