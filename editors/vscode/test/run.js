/** Install the packaged `.vsix` into a downloaded VS Code and run `suite.js` in it.
 * Needs `natural-syntax-ls` on `PATH` and a display (e.g. `xvfb-run`). */
const { spawnSync } = require("node:child_process");
const { mkdirSync, mkdtempSync, writeFileSync } = require("node:fs");
const { tmpdir } = require("node:os");
const { join, resolve } = require("node:path");
const {
    downloadAndUnzipVSCode,
    resolveCliArgsFromVSCodeExecutablePath,
    runTests,
} = require("@vscode/test-electron");
const { name, version } = require("../package.json");

async function main() {
    const vscodeExecutablePath = await downloadAndUnzipVSCode();
    const tmp = mkdtempSync(join(tmpdir(), `${name}-`));
    const dirArgs = [
        `--extensions-dir=${join(tmp, "extensions")}`,
        `--user-data-dir=${join(tmp, "user")}`,
    ];
    mkdirSync(join(tmp, "user", "User"), { recursive: true });
    writeFileSync(
        join(tmp, "user", "User", "settings.json"),
        JSON.stringify({ "naturalSyntaxLs.tokenMapUpdate": { DT: null } }),
    );
    const [cli, ...cliArgs] =
        resolveCliArgsFromVSCodeExecutablePath(vscodeExecutablePath);
    const vsix = resolve(__dirname, "..", `${name}-${version}.vsix`);
    const install = spawnSync(
        cli,
        [...cliArgs, ...dirArgs, "--install-extension", vsix],
        { stdio: "inherit" },
    );
    if (install.status !== 0) throw new Error(`Installing ${vsix} failed.`);
    await runTests({
        vscodeExecutablePath,
        extensionDevelopmentPath: resolve(__dirname, "empty_extension"),
        extensionTestsPath: resolve(__dirname, "suite.js"),
        launchArgs: [...dirArgs, "--no-sandbox", "--disable-gpu"],
    });
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
