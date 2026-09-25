// A test child's whole environment, built from nothing (ticket 0127).
// The child gets PATH, the parent's value of each name the caller keeps, and
// the values the caller sets. A secret-shaped or THINKTHEN_ name is never kept.
const SECRET = /^THINKTHEN_|KEY|TOKEN|SECRET|PASSWORD|CREDENTIAL|AUTH/i;

/** PATH, each kept name the parent has, then the values set here. */
export function childEnv({ keep = [], values = {} } = {}) {
  const env = { PATH: process.env.PATH ?? '/usr/bin:/bin' };
  for (const name of keep) {
    if (SECRET.test(name)) {
      throw new Error(`a test child may not keep ${name} from the parent: set a THINKTHEN_ value or a fake key explicitly`);
    }
    if (process.env[name] !== undefined) env[name] = process.env[name];
  }
  return { ...env, ...values };
}
