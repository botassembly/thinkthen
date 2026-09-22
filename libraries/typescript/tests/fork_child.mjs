// The child of the fork test: a fresh process (fork + exec) that makes
// its own call after the parent's first call, and reports over IPC.

import * as tt from '../index.mjs';

const answer = await tt.decide('Does the customer ask for a refund?', 'I want a refund for order 9');
process.send({ answered: answer === true });
process.exit(0);
