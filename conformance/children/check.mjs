// The TypeScript helper's proof, run by test.sh with sentinels in this process.
import { spawnSync } from 'node:child_process';

import { childEnv } from './children.mjs';

const PROBE = 'test -z "${THINKTHEN_SENTINEL+x}" && test -z "${FAKE_SERVICE_API_KEY+x}" && '
  + 'test -z "${ABSENT_0127+x}" && test "$KEPT_0127" = kept && test "$SET_0127" = set';
const REFUSED = ['THINKTHEN_BASE_URL', 'OPENAI_API_KEY', 'GITHUB_TOKEN', 'db_password', 'AWS_SECRET_ACCESS_KEY'];
const sentence = (name) => `a test child may not keep ${name} from the parent: set a THINKTHEN_ value or a fake key explicitly`;

const env = childEnv({ keep: ['KEPT_0127', 'ABSENT_0127'], values: { SET_0127: 'set' } });
const bad = spawnSync('sh', ['-c', PROBE], { env: env }).status === 0 ? [] : ['the child'];
for (const name of REFUSED) {
  try {
    childEnv({ keep: [name] });
    bad.push(name);
  } catch (error) {
    if (error.message !== sentence(name)) bad.push(error.message);
  }
}
console.log(`children typescript: ${bad.length ? `FAIL ${bad.join(', ')}` : 'ok'}`);
process.exit(bad.length ? 1 : 0);
