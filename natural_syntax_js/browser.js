import { createPOSModel } from './index.js';

const form = document.querySelector('form');
const button = document.querySelector('button');
const status = document.querySelector('#status');
const output = document.querySelector('#result');
const loaded = await createPOSModel();
status.textContent = loaded.ok ? 'Ready' : loaded.error;
button.disabled = !loaded.ok;
form.addEventListener('submit', async event => {
    event.preventDefault();
    button.disabled = true;
    status.textContent = 'Tagging…';
    const result = await loaded.value.predict(document.querySelector('#text').value);
    output.textContent = result.ok ? JSON.stringify(result.value, null, 2) : '';
    status.textContent = result.ok ? 'Ready' : result.error;
    button.disabled = false;
});
