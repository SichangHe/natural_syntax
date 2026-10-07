import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';

const child = spawn(process.argv[2] ?? 'natural-syntax-ls', [], {
  stdio: ['pipe', 'pipe', 'inherit'],
});
const pending = new Map();
let buffer = Buffer.alloc(0);
let nextId = 0;
const deadline = setTimeout(() => {
  child.kill();
  throw new Error('Language server did not finish within 120 seconds');
}, 120_000);

function send(method, params, id) {
  const body = JSON.stringify({ jsonrpc: '2.0', method, params, ...(id === undefined ? {} : { id }) });
  child.stdin.write(`Content-Length: ${Buffer.byteLength(body)}\r\n\r\n${body}`);
}

function request(method, params) {
  return new Promise((resolve, reject) => {
    const id = ++nextId;
    pending.set(id, { resolve, reject });
    send(method, params, id);
  });
}

child.stdout.on('data', chunk => {
  buffer = Buffer.concat([buffer, chunk]);
  for (;;) {
    const end = buffer.indexOf('\r\n\r\n');
    if (end < 0) return;
    const length = Number(/Content-Length: (\d+)/i.exec(buffer.subarray(0, end).toString())?.[1]);
    assert.ok(Number.isInteger(length));
    if (buffer.length < end + 4 + length) return;
    const message = JSON.parse(buffer.subarray(end + 4, end + 4 + length));
    buffer = buffer.subarray(end + 4 + length);
    const reply = pending.get(message.id);
    if (reply) {
      pending.delete(message.id);
      if (message.error) reply.reject(new Error(JSON.stringify(message.error)));
      else reply.resolve(message.result);
    }
  }
});
child.on('error', error => {
  for (const reply of pending.values()) reply.reject(error);
});
child.on('exit', code => {
  for (const reply of pending.values()) reply.reject(new Error(`Server exited: ${code}`));
});

try {
  const initialized = await request('initialize', { capabilities: {} });
  assert.ok(initialized.capabilities.semanticTokensProvider);
  send('initialized', {});
  const uri = 'file:///natural-syntax-smoke.txt';
  const text = '😀 The birds fly.\r\nCafé birds sing.';
  send('textDocument/didOpen', {
    textDocument: { uri, languageId: 'plaintext', version: 1, text },
  });
  const tokens = await request('textDocument/semanticTokens/full', { textDocument: { uri } });
  assert.ok(tokens.data.length > 0);
  assert.equal(tokens.data.length % 5, 0);
  let line = 0;
  let column = 0;
  const lines = text.split('\r\n');
  for (let i = 0; i < tokens.data.length; i += 5) {
    const [deltaLine, deltaStart, length] = tokens.data.slice(i, i + 3);
    column = deltaLine ? deltaStart : column + deltaStart;
    line += deltaLine;
    assert.ok(column + length <= lines[line].length);
    const word = lines[line].slice(column, column + length);
    assert.ok(['😀', 'The', 'birds', 'fly', 'Café', 'sing'].includes(word), word);
  }
  send('textDocument/didChange', {
    textDocument: { uri, version: 2 }, contentChanges: [{ text: '' }],
  });
  const empty = await request('textDocument/semanticTokens/full', { textDocument: { uri } });
  assert.deepEqual(empty.data, []);
  await request('shutdown');
  send('exit');
} finally {
  clearTimeout(deadline);
  child.kill();
}
