# ThinkThen for TypeScript and JavaScript

The public release remains 0.1.2. The owned API below requires a development 0.2 package.

```sh
npm install thinkthen
```

```typescript
import {Client} from 'thinkthen';
const client = new Client({cache: false});
try {
  const call = await client.decide('Does the writer request a refund?', 'Please refund my order.');
  console.log(call.results[0].value);
} finally { client.close(); }
```

CommonJS uses `const {Client} = require('thinkthen')`. Both module forms expose the same ten named Client functions: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Results and declarations are generated from Rust. `results` retains completed answers and `facts` retains final native observations. Missing properties remain distinct from present null; `.has(name)` reports presence. Unknown result members remain available. `ClientError` carries native kind, retryability, failure facts and completed results.

Use `Client.item(value, fields)` for per-record context, options and image descriptors, `Client.files(paths, reading, media, framing)` for explicit native sources, and `Client.questionFile`, `Client.questionName` or `Client.questionReference` for saved questions. Rust owns admission and settings. Settings and call options use the native snake_case names in the [Request contract](../../specification/request.schema.json).

Named calls return Promises. `client.start(function, question, input, controls)` returns an async packet iterator with `result()`, `cancel()` and `close()`. Iterable and async iterable inputs are bounded by the native session. `controls.signal` accepts an AbortSignal. Closing or cancelling releases intake and returns while a provider reply is held; final facts remain pending until native settlement.

The old `Engine`, default functions, `complete`, `*_many`, `details`, `plan`, copied CompleteTypes and answer helpers are retired. Replace `engine.complete.decide(...)` or `engine.decide(...)` with `client.decide(...)`; read `call.results[0].value` instead of `call.value`. Replace batch methods with `client.start(...)`. Use generated `Results` types instead of CompleteTypes.

The package loads its bundled addon automatically. The generated [platform inventory](native-platforms.json) includes Windows x64. Windows execution, registry installs and full parity run at the candidate.

`setup-toolchain.sh` installs the pinned Node toolchain. `build-addon.sh` builds the addon offline. `check.sh` checks the local source package or an installed tarball named by `THINKTHEN_ARTIFACT`, using only loopback providers. Shared routine cases run through JavaScript and compiled TypeScript consumers.
