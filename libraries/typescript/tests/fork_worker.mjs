// The worker of the fork test: a thread sharing the parent's process and
// its addon, answering after the parent's first call.

import { parentPort } from 'node:worker_threads';

import * as tt from '../index.mjs';

const answer = await tt.decide('Does the customer ask for a refund?', 'I want a refund for order 9');
parentPort.postMessage({ answered: answer === true });
