// A test child's whole environment, built from nothing (ticket 0127).
// The child gets PATH, the parent's value of each name the caller keeps, and
// the values the caller sets. A secret-shaped or THINKTHEN_ name is never kept.
import { join } from 'node:path';

const SECRET = /^THINKTHEN_|KEY|TOKEN|SECRET|PASSWORD|CREDENTIAL|AUTH/i;

/** PATH, kept names, optional owned product folders, then explicit overrides.
 * Tool children omit home. Supplying folders does not create them. */
export function childEnv({ keep = [], home, values = {} } = {}) {
  const env = { PATH: process.env.PATH ?? '/usr/bin:/bin' };
  for (const name of keep) {
    if (SECRET.test(name)) {
      throw new Error(`a test child may not keep ${name} from the parent: set a THINKTHEN_ value or a fake key explicitly`);
    }
    if (process.env[name] !== undefined) env[name] = process.env[name];
  }
  if (home !== undefined) Object.assign(env, {
    HOME: home,
    XDG_CONFIG_HOME: join(home, 'config'),
    XDG_CACHE_HOME: join(home, 'cache'),
    XDG_STATE_HOME: join(home, 'state'),
    APPDATA: join(home, 'config'),
    LOCALAPPDATA: join(home, 'local'),
  });
  return { ...env, ...values };
}
