# 0498: Make TypeScript thin and first-class

Status: OPEN.

Milestone: 0.2

Depends on: 0511
Depends on: 0513
Depends on: 0501

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

Reviews: revision 36382970a, accept

Reviews: revision 1e3f62bf0bb0a432af7f41f4dda5468937946439, accept

## Outcome

A TypeScript or JavaScript caller installs the npm package, imports it as CommonJS or ES modules, calls the ten functions by name with ordinary values, and gets typed results at runtime and in the editor. Async calls keep the event loop responsive and cancel cleanly. Rust owns every rule and observation; the addon keeps only naming, conversion, streams, cancellation and cleanup.

## Evidence

- Starts from: the [2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). The TypeScript adapter repeats admission and result construction, including settings, batch and deadline checks. Its hand-copied reader in `_complete.js` and `_complete.d.ts` lacked facts fields added by 0461 and 0468; 0520 repaired it as a narrow bridge and its reproduced refusal cases stay.
- Keeps: Named addon methods, both module forms, async streams and cancellation. All ten functions and their input, result, error, cache and replay behavior. Missing stays distinct from null, and permitted unknown result fields are tolerated. Each existing platform's addon name and bytes, and the refusal sentence for a platform the package does not ship.
- Changes: Meet the caller acceptance and the TypeScript section of `../../libraries/BINDING-AUTHOR.md`. This ticket owns:
  - the TypeScript target template from 0513's common graph, generating both the declarations and the actual runtime conversion;
  - conversion of JavaScript values into the shared Request, with no restated checks;
  - typed errors carrying Rust error kinds and facts;
  - Promise and async-stream execution, cancellation and close;
  - the npm packaging slice under 0517's design: build the `win32-x64` addon beside the existing addons, accept it in `loader.js` and `package.json`, add a Windows Node pin, derive the addon list from 0501's inventory with no second literal addon count, and turn the `check.sh` `win32` refusal into a load case;
  - the package README, with a short old-to-new call mapping and Windows named as supported;
  - removal of the old public names and copied readers after installed parity.
  One public API is one coherent family of named typed calls. Claim `libraries/typescript/**` and its installed typed consumer cases, narrowed per slice before coding.
- Proof: The full shared cases run through the installed package in both module forms, including files and images, context and options, original positions, facts, failures and invalid input with zero sends. One installed held-provider case keeps the event loop responsive and shows cancellation or close stopping further reads and submissions before the provider is released. Pending final facts stay pending. Declarations or a Promise return type alone do not count, and neither does raw JSON pass-through. Record handwritten code removed and added, counting generator templates, in the landing record.
- Defers: Native Windows qualification goes to 0383 at the first authorized candidate. Final assembly goes to 0530. The proxy and platform ruling changes need no 0.2 ticket.

## Progress

- 2026-10-10 landed 84de81b66; next: The recorded refusal bug is already fixed by 61c9154e2 and 84de81b66. Seventeen existing refusal, routing, profile and secrecy cases pass against current JavaScript and the warm addon; no new code was needed. Final current installed parity and Windows qualification remain held.
