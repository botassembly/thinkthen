// A child process started after its parent's first call makes its own call.
import * as tt from '../index.mjs';

process.send({ answered: (await tt.decide('Does the customer ask for a refund?', 'child text')).value });
process.disconnect();
