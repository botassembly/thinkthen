# 0739: Run find probe checks with the supplied installed command

Status: COMPLETE.

Milestone: 0.2.1

Phase: 1

Reviews: revision 7ee93b6, accept

Landed: 3236f1f

## Outcome

- Changes: `probes/find-0040/self-test`, `probes/probability-total-0038/self-test`.
- Both probes prefer the executable supplied by the existing package/spec caller, with the existing repository convention for a standalone invocation. It runs in a fresh package checkout without a debug command at target/debug/thinkthen.

## Keeps

- Preserve every probe assertion, fixture and native behavior. Add no helper, mode, provider, deadline change or executable copy.

## Proof

- Reproduce the missing-command failure through the existing probe command with an installed command supplied and no repository debug command. Show that the repaired command passes with the same assertions, and a stale repository debug command cannot override the supplied installed command.
- The configured landing gate passes on the integrated revision.

## Evidence

Gator checkpoint0691/1792505591223-604460 on969e17b5444bea3544e5549f8078a58a1e3c1026 exited127. Package reached243 installed CLI cases with zero failures, then probes/find-0040/self-test:223 tried the absent repository target/debug/thinkthen. Line228 uses the same path. The operator changed no source and did not rerun.
