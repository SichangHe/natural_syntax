const assert = require("node:assert");
const { commands, window, workspace } = require("vscode");

const MODEL_LOAD_TIMEOUT_MS = 600_000;
const POLL_INTERVAL_MS = 1_000;

/** Semantic tokens of `uri` once the server has registered its provider. */
async function semanticTokens(uri) {
    const deadlineMs = Date.now() + MODEL_LOAD_TIMEOUT_MS;
    while (Date.now() < deadlineMs) {
        const tokens = await commands.executeCommand(
            "vscode.provideDocumentSemanticTokens",
            uri,
        );
        if (tokens?.data.length) return tokens.data;
        await new Promise((resolve) => setTimeout(resolve, POLL_INTERVAL_MS));
    }
    throw new Error("No semantic tokens before timeout.");
}

/** Entry point VS Code calls: the installed extension highlights plain text and
 * honors `tokenMapUpdate` (`run.js` disables determiners). */
async function run() {
    const document = await workspace.openTextDocument({
        language: "plaintext",
        content: "The dog barks loudly.",
    });
    await window.showTextDocument(document);
    const data = await semanticTokens(document.uri);
    console.log("Semantic tokens:", Array.from(data).join(" "));
    assert.strictEqual(data.length % 5, 0);
    assert.strictEqual(data[0], 0, "First token is on the first line.");
    assert.strictEqual(data[1], 4, "`The` is disabled, so `dog` is first.");
    assert.strictEqual(data[2], 3, "`dog` is 3 characters long.");
}

module.exports = { run };
