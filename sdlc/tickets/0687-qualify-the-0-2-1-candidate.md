# 0687: Qualify the 0.2.1 candidate on one frozen main revision

Status: OPEN.

Milestone: 0.2.1

Phase: release

Depends on: 0564, 0617, 0620, 0621, 0624, 0630, 0631, 0632, 0634, 0640, 0644, 0645, 0646, 0647, 0649, 0652, 0654, 0656, 0660, 0661, 0663, 0664, 0666, 0669, 0670, 0672, 0674, 0675, 0677, 0680, 0681, 0684, 0685, 0690, 0691, 0694, 0695, 0696, 0711, 0592, 0710, 0712, 0713, 0714, 0715, 0716, 0717, 0718, 0722

## Outcome

- Changes: `crates/thinkthen/Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`, `release-notes/0.2.1.md`, `sdlc/records/0687-qualification.md`, and the package manifests that `sdlc/scripts/versions --set 0.2.1` writes.
- The 0.2.1 candidate is one main revision that has passed the 0691 checkpoint run, tagged `rc/0.2.1-rc.N`, the only candidate ref the release workflow's resolve step admits (0620). Every language library builds natively and passes its installed tests on every platform it supports. The rehearsal and the hosted Linux core and functional jobs pass on that same revision, with no release-only check meeting it for the first time.
- Each item above has a named passing receipt on that revision: the job name and log for each item and each hosted job, all on one SHA. In particular:
  - the hosted Linux x86_64 full-case jobs pass with no timeout (0621);
  - the hosted Windows recognition-session cases pass repeatedly, or each remaining failure is filed and ruled on before the go;
  - `install.ps1` installs the real Windows zip and runs the command (0632);
  - the Ruby platform gem installs and runs on Ruby 3.4 and 4.0 (0592);
  - the Windows x64 NuGet consumer (0669), the Go windows/amd64 build (0657), and the C, C++ and Python Windows and Intel macOS installs (0641, 0653, 0660) pass.
- The 0425 artifact QA control job (branch `ticket/0425-hosted-artifact-qa`, retargeted and reviewed) runs on the tagged candidate and measures both C++ 100000-record memory modes. Its measurement keeps test-fixture copy and serialize memory separate from native held-answer memory.
- Package release numbers read 0.2.1, set with `sdlc/scripts/versions --set 0.2.1`. Format versions stay removed, per `sdlc/decisions/2026-10-19-no-version-numbers-before-1-0.md`. The coordinator brings the passing revision to the PM for Ian's publish go.

### Proof

- One list in `sdlc/records/0687-qualification.md` names every required receipt above by job and log, each on the same candidate SHA. A missing or failing receipt fails this ticket.
- On the M5: `pm job run --machine m5 0687 SHA --memory 16 -- sh -c 'sdlc/scripts/test-full-cases --run && sdlc/scripts/package && sdlc/scripts/test-stress --run'` passes on the candidate SHA. The Linux arm64 and x86_64 runs come from the 0619 containers on the M5 and Yellow; Windows and Intel macOS come from the hosted rehearsal (`gh workflow run release.yml --ref rc/0.2.1-rc.N -f mode=rehearse`) and `windows.yml` on the same tag.

## Evidence

0.2.0 was never qualified or published. Its last candidate, 393f4bec1, passed the full M5 lint and test, and the hosted Windows, Swift and Objective-C jobs. The rehearsal was cancelled after resolve, crate and Windows build, and the Linux job was cancelled during tool installation. Several repairs on main have only partial proof:
- Polars preparation (3ef6630e7): passed on the M5, not on Linux.
- C# separate build output (cc6335e1d): not run where .NET is installed.
- Flutter archived source (a30402a25): its Linux packaging fixture was skipped on the M5.
- Windows Scala classpath (f5bb0db8c): the full Windows SDK smoke has not run.
- Apple DuckDB error category (e69638b92): the final-platform installed smoke has not run.
- Public README replay preparation (2c3a11bdd): the full Linux replay has not run.
- Intel smoke host reuse (65acd9a5e, replaced by 0613): the restore and the complete smoke have not run.
- Linux image timeouts: measured and fixed on Yellow in 0621, not yet passed on the hosted Linux job.
- Windows recognition-session timeouts: hosted Windows run 38666072874 timed out waiting for duplicate-recognition cancellation to settle, and the 0543 evidence record keeps the measured rate. They are intermittent and unresolved.

Everyday tickets prove only the M5, the M5 arm64 container and the Yellow x86_64 container. Windows, Intel macOS and the final hosted Linux jobs run only here.
