const { workspace } = require("vscode");
const { LanguageClient } = require("vscode-languageclient/node");

let client;

/** Start `natural-syntax-ls` over stdio for plain text documents. */
function activate() {
    const config = workspace.getConfiguration("naturalSyntaxLs");
    client = new LanguageClient(
        "naturalSyntaxLs",
        "Natural Syntax LS",
        { command: config.get("serverPath") },
        {
            documentSelector: [{ language: "plaintext" }],
            initializationOptions: {
                token_map_update: config.get("tokenMapUpdate"),
            },
        },
    );
    return client.start();
}

function deactivate() {
    return client?.stop();
}

module.exports = { activate, deactivate };
