
## usvg for JavaScript

An svg simplifier from resvg.

```js
import initUSvg, { optimize } from "usvgjs";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const wasmFile = require.resolve("usvgjs/usvgjs_bg.wasm");
const wasmBuffer = await fs.readFile(wasmFile);

await initUSvg({ module_or_path: wasmBuffer });
const svg = optimize('<svg...>')
```