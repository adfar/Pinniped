import init, { parse, to_markdown } from '../wasm-bindings/pkg/pinniped_wasm.js';

async function run() {
    await init();
    const input = document.getElementById('markdown-input');
    const output = document.getElementById('output');
    document.getElementById('parse-button').addEventListener('click', () => {
        const result = parse(input.value);
        const roundTrip = to_markdown(result);
        output.textContent = roundTrip;
    });
}

run();
