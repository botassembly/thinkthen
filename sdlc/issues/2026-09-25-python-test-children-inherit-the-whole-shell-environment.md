# Python test children inherit the whole shell environment

Status: Open

Filed on 2026-09-25 from the pandas build (ticket 0122). No key was used, and no request left the machine.

## What happens

`child_env` in `libraries/python/tests/conftest.py` copies the parent's whole environment, removes `THINKTHEN_API_KEY`, and sets the fake key. Every other variable of the developer's shell reaches each test child, including unrelated service keys. Two paths then print them:

- pytest's default traceback shows the `env` argument of `run` when a child exits nonzero.
- The secrecy test's planted bug formats the child's environment into an error message. The test turns red as it should, and its assertion diff prints every inherited variable.

The 0122 build ran pytest with `--tb=native` to keep the first path quiet. The second path printed unrelated keys from the builder's shell into the plant run's output. That output lived only in the session's scratch folder and was deleted.

## What would fix it

Build the child's environment from a short allow list: `PATH`, `HOME`, `LANG`, the toolchain variables the tests need, then the fake key, the loopback address, and the cache folder. The secrecy tests keep their meaning, since the fake key still reaches the child.

## Tests to update when fixed

- `libraries/python/tests/test_secrecy.py`, `test_the_child_environment_holds_the_fake_key_beside_loopback`: also assert that a sentinel variable set in the parent does not reach the child.
