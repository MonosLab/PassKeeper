// 1. Install esbuild package: npm install @tauri-apps/api esbuild --save-dev
// 2. Bundle: npx esbuild bundle.js --bundle --format=esm --outfile=src/tauri-api.js
// 3. In main.js, call function via invoke: window.tauriInvoke.functionName
// 4. Declare functions globally for use in HTML.
//    Example: window.functionName = functionName;
// 5. Load tauri-api.js and main.js in HTML
//    <script type="module" src="./tauri-api.js"></script>
//    <script type="module" src="./main.js"></script>

import { invoke } from '@tauri-apps/api/core';
window.tauriInvoke = invoke;