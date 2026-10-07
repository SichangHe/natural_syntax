#!/usr/bin/env node
import { createPOSModel } from './index.js';

const args = process.argv.slice(2);
if (args.includes('--help')) {
    console.log('Usage: natural-syntax [TEXT]\nWithout TEXT, read UTF-8 text from stdin. Print tagged words as JSON.');
} else {
    let input = args.join(' ');
    if (!args.length) {
        process.stdin.setEncoding('utf8');
        for await (const chunk of process.stdin) input += chunk;
    }
    const loaded = await createPOSModel();
    if (!loaded.ok) {
        console.error(`Loading model: ${loaded.error}`);
        process.exitCode = 1;
    } else {
        try {
            const result = await loaded.value.predict(input);
            if (result.ok) console.log(JSON.stringify(result.value));
            else { console.error(result.error); process.exitCode = 1; }
        } finally { await loaded.value.dispose(); }
    }
}
