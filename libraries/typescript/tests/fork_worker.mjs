// A worker thread that loads the addon beside its parent and makes its own call.
import { parentPort } from 'node:worker_threads';

import * as tt from '../index.mjs';

parentPort.postMessage({ answered: (await tt.decide('Does the customer ask for a refund?', 'worker text')).value });
