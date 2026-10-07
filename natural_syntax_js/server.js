#!/usr/bin/env node
import { createPOSModel } from './index.js';
import { semanticTokens, tokenMap, TOKEN_MODIFIERS, TOKEN_TYPES } from './tokens.js';

// 🧑 “And make a JS version of the idea.”
/**
 * Language server over stdio that highlights parts of speech with semantic tokens.
 * Same LSP surface as the Rust `natural-syntax-ls`.
 */
const loading = createPOSModel();
/** URI → `{text, version, pending, tagging, words, tagged_text}`. */
const documents = new Map();
let map = tokenMap().value;

function send(message) {
    const body = JSON.stringify({ jsonrpc: '2.0', ...message });
    process.stdout.write(`Content-Length: ${Buffer.byteLength(body)}\r\n\r\n${body}`);
}

/**
 * Tag the latest text of `document` until no newer text is pending:
 * only one prediction per document runs at a time, and the latest update replaces queued ones.
 */
async function tag(document, model) {
    while (document.pending) {
        const text = document.text;
        document.pending = false;
        const result = await model.predict(text);
        if (!result.ok) console.error(`Tagging text: ${result.error}`);
        [document.words, document.tagged_text] = [result.ok ? result.value : [], text];
    }
    document.tagging = undefined;
}

async function change(uri, version, text) {
    const document = documents.get(uri) ?? { version: -Infinity };
    documents.set(uri, document);
    if (version <= document.version) return;
    Object.assign(document, { text, version, pending: true });
    const loaded = await loading;
    if (loaded.ok) document.tagging ??= tag(document, loaded.value);
}

const handlers = {
    initialize({ initializationOptions }) {
        const updated = tokenMap(initializationOptions?.token_map_update ?? {});
        if (updated.ok) map = updated.value;
        else send({ method: 'window/logMessage', params: { type: 1, message: `Invalid initialization options: ${updated.error}` } });
        return { capabilities: {
            textDocumentSync: 1,
            semanticTokensProvider: { legend: { tokenTypes: TOKEN_TYPES, tokenModifiers: TOKEN_MODIFIERS }, full: true },
        } };
    },
    'textDocument/didOpen': ({ textDocument: { uri, version, text } }) => change(uri, version, text),
    'textDocument/didChange': ({ textDocument: { uri, version }, contentChanges }) =>
        change(uri, version, contentChanges.at(-1).text),
    'textDocument/didClose': ({ textDocument: { uri } }) => void documents.delete(uri),
    async 'textDocument/semanticTokens/full'({ textDocument: { uri } }) {
        await loading;
        const document = documents.get(uri);
        await document?.tagging;
        return document?.words ? { data: semanticTokens(document.tagged_text, document.words, map) } : null;
    },
    shutdown: () => null,
    exit: () => process.exit(0),
};

async function handle({ id, method, params }) {
    const handler = handlers[method];
    if (id === undefined) return handler?.(params ?? {});
    if (!handler) return send({ id, error: { code: -32601, message: `Method not found: ${method}` } });
    send({ id, result: await handler(params ?? {}) });
}

let buffer = Buffer.alloc(0);
process.stdin.on('data', chunk => {
    buffer = Buffer.concat([buffer, chunk]);
    for (;;) {
        const end = buffer.indexOf('\r\n\r\n');
        const length = Number(/Content-Length: (\d+)/i.exec(buffer.subarray(0, Math.max(end, 0)).toString())?.[1]);
        if (end < 0 || buffer.length < end + 4 + length) return;
        const message = JSON.parse(buffer.subarray(end + 4, end + 4 + length));
        buffer = buffer.subarray(end + 4 + length);
        if (message.method) handle(message);
    }
});
process.stdin.on('end', () => process.exit(0));
const loaded = await loading;
if (!loaded.ok) {
    console.error(`Loading model: ${loaded.error}`);
    process.exit(1);
}
