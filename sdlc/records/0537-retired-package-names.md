# Current package names and migration wording

The JVM package builds and ships its typed code through the `session` module and `thinkthen-session.jar`. Maven coordinates, package inventories, installed callers and managed release fixtures use the current names. `Json.java` remains necessary for Engine, NativeSession and NativeLoader, so it moved under the session module without changing its `thinkthen` package or public class. Frozen C exports remain unchanged.

COBOL now has the required old-to-new call and carrier mappings and retains its approved scalar representation limit. The Foundation upgrade text describes the API without test status. Current binding wording and a stale TypeScript settings-source path were corrected. The site's 25 language entries link their package guides and existing checked consumers; obsolete host file examples were removed instead of recreating the deleted site examples.

A fresh code reviewer accepted `301c02dc04c14efa330bc7dd38e41260615bd772` after reading the JVM rename, active JSON callers, package inventories, Maven and release callers, generated header path and site links. Changed source gained 30 nonblank lines: the site example mapping added 32 and duplicate source appends removed two. This is not a claim that moving Java source deleted it. JVM Python source and its ceiling fell from 649 to 648; Java 4001, Kotlin 1658 and Scala 1672 remain unchanged.

The installed JVM checks passed for Java, Kotlin and Scala through both development JARs and local Maven coordinates. All nine usage persistence cases and package-content refusal checks passed. The managed-pair release self-test and 40 registry self-tests passed. The site built and its 195-page internal-link check and package link-target checks passed. Policy, JVM source ceilings and closure lint passed. The core Rust and routine supporting code are unchanged; the uninterrupted 0535 routine suite remains applicable. Its installed checks are supplemented by the affected JVM and release checks here. All processes exited and were awaited.

`target/0537-retired-package-names/` in the main checkout retains the build, installed consumer, release fixture, policy, search and site logs, package inventory and closure lint log. The retained native package and command were reused without changing their caches. Build output is not pushed.

## What the build taught us

The old directory name did not make every file inside it obsolete. Deleting the actively used JSON codec would have broken the typed package; moving it preserved its callers. The shipped-name search distinguishes current API references from the previous-call labels required by migration tables and from forbidden-name assertions in package tests. Those historical labels document removal and do not revive a retired API.

The site links the checked consumer owned by each package instead of maintaining a second example. Link checks cover the added destinations. Renaming the JVM module also required updating the registry release fixture, not only the managed-pair fixture named in the ticket.
