# 0469: Reconcile issue scopes with landed fixes

Starting revision: `e7280ba3c`. This changes issue records and one existing audit link. It changes no runtime behavior and supplies no new platform or release qualification.

## Verified dispositions

| Earlier issue | Disposition and exact reference |
| --- | --- |
| Bindings/SQL cannot name backends | Closed through pm against `0f263872a` (0377); native and host settings preserve the selected route. |
| SQLite find drops its model | Closed through pm against `f22c092ca` (0433); `scalars/find.rs` applies `with_model`. |
| pandas Series / DuckDB kind descriptions | Closed through pm against final integration record `e056fa21c`; runtime landings are `730aa68ed` (0410) and `7b3f7729b` (0434/0435). |
| Polars deadline fixture | Closed through pm against `730aa68ed` (0410); the fixture waits for one send only when a send occurs, retains expired-no-send and mid-batch cancellation cases. Debt 036 carries its paid date. |
| Release tag differs from rehearsal | Closed through pm against `68abc60a4` (0398 C); exact resolved-commit admission does not establish a successful candidate run. |
| Rank details retain null value | Closed through pm against `ffb1b253d`; result/2 uses the final one-based rank position. |
| Four already-closed files in the open folder | Moved through pm: native install to `88e5ab1e1` (0437), extract/verify/cache recipes to `94c06a551` (0402 E). Original draft metadata is historical text. |
| Every surface's run facts | Narrowed to 0425 qualification. The `60f0dcb9a` installed campaign, landed at `0553becc7`, covers the five implemented parts; missing provider timing remains absent. |
| Search positions, sets and display | Narrowed to SDK contract adoption and 0425 qualification. `57de974e8` supplies display, `ffb1b253d` CLI positions, `b835816b6` typed members and `0d78347db` foreign adoption. Current top-after-merge semantics supersede the proposal. |
| R installation / public install checks | Narrowed to actual qualification and source/publication sequencing. `715a6a994` supplies Linux documentation; `dda236c21` supplies install workflow/guard; `ddfcbc74c` fixes R contribution paths. |
| Rehearsal publish checks | Narrowed to account and hosted qualification. `065a4e5dd` checks supported publish inputs; 0398 supplies offline safety. |
| Ruby inert fallback | Narrowed to held public registry remediation/publication. `e2ec4deb6` builds the diagnostic fallback; `60f0dcb9a` qualifies candidate Ruby behavior. |
| Ambient spec/demo configuration | Narrowed to 0487's remaining child environments. `ed7f5d9fa` (0471) adds `config_home` to both owning runners. |

The closed issue files retain historical failures. The remaining issue files retain genuine obligations. [0432 qualification](0432-shared-parity-cases.md#final-installed-qualification-2026-10-08) and the owning records preserve executed evidence; no old proof was reconstructed here.

The release/install issue now distinguishes completed 0.1 publication under 0128 from 0.2 platform and public-package obligations under 0425/0398. Windows cache admission and replay/conversion coexistence remain owed under 0474/0480. No tag, workflow dispatch, release-branch advancement, publication, registry remediation or paid call follows from this record. Ian's release hold remains in force.

The silent SDK/SQL usage writer defect remains open in 0.2 under 0468. Transcript record/accounting help and warning choices remain open; current filter-versus-rank guidance is recorded. Checkpoint doc-test enforcement and strict release proof remain open, separate from exact-commit rehearsal routing. The install-copy obligation now points to 0425 rather than completed core docs ticket 0402.

The rank-threshold idea retains Ian's 2026-10-01 0.2 ruling and names its conflict with the settled rank contract for the PM. Cleanup does not revoke that ruling. Scalar SQL naming and CLI described score levels remain proposals. Link/navigation and pathology demonstration remain later; duplicate draft headers are historical fields. Four retained planning-intake ideas gain their missing later milestone. Proxy scope uses 0.3. Upstream adapter workarounds, lazy-streaming debt, relation expansion and Windows static-library scope remain intact.

The pm closure automatically changed the two-gap issue link in [0405 audit](0405-audit-report.md). The coordinator authorized this single link-only exception to the historical-record boundary.

## Checks

`python3 sdlc/scripts/tickets` passed with zero failures. `python3 sdlc/scripts/pages` passed for 27 existing pages. The existing pages link reader also checked the changed Markdown files and found no missing local targets. `git diff --check` passed. `pm lint records` and `pm lint links` each reported zero findings but explicitly disabled those families under the light flow; they supply no active validation claim. Issue header and milestone dispositions were compared directly with their prior scope and source references. No build or runtime test ran, as required by the ticket.

## What the build taught us

A landed implementation, an installed local candidate and a qualified public package answer different questions. Issue scopes should cite the implementation fix and retain the narrower platform or publication obligation. Old draft metadata must read as history so pm sees one current disposition. Existing link rewriting avoids broken issue references, but its historical-record changes need an explicit ownership exception.
