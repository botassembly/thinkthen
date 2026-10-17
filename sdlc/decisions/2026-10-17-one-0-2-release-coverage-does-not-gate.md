# One 0.2 release; coverage does not gate it

Ian, 2026-10-17: 0.2 ships as one release, with no 0.2.x follow-up release for languages that miss it. Work through the remaining issues until every language library compiles and passes its tests on every platform it supports.

- Code coverage does not gate 0.2. The hosted coverage job leaves the 0.2 release qualification, and no 0.2 freeze or tag waits on it or on the Apple artifact it needs. Coverage measurement resumes after 0.2 ships.
- Release qualification still requires native builds and installed tests for every language library, the rehearsal, and the hosted Linux core and functional jobs, all on one frozen revision before the tag.
- The 0.2.1 milestone stays a docs and site update that needs no new release.
