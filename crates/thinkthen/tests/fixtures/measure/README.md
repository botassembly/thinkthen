# Measure fixtures

These files define `thinkthen audit` and `thinkthen diff`. The source is a prototype measurement script at commit `be7cea2e4aa41097e7f629e35b62dadedeaca544`. Its three files had these SHA-256 values at that commit:

| Prototype file | SHA-256 |
| --- | --- |
| `scripts/tools/measure.py` | `7be206812f0e41011a9b31c59030c480f8b6bed945d7976db452f3908f0316a1` |
| `tests/test_measure.py` | `d64f2d9088c4f6237c0d2a2a8faf06dc51f92cd4674dab20b2a4f7231c01fa4e` |
| `scripts/tools/README.md` | `fae695523a16d1f2921ee9c73358c36e1613b58ee6acc9d720c6bc6e1d9e7fc7` |

`small/`, `249/`, and the fourteen files directly under `golden/` are byte-for-byte copies of the prototype's `tests/fixtures/audit/` at that commit. `249/` holds `thinkthen decide --details` lines replayed from an earlier experiment's recordings with every key unset: 272 yes/no questions about Beatles songs.

Three inputs are derived from `small/`. `decide-reversed.jsonl` holds the lines of `decide.jsonl` in reverse. `decide-key-noparts.jsonl` holds the key without `part`. `decide-key-odd.jsonl` holds the key without `part` and without `r6`.

The files under `golden/extra/`, `golden/table/`, and `replay/audit.jsonl` were captured from the prototype script at that commit, read with `git show` into a scratch file and run under Python 3.12.3 from this folder. `replay/key.jsonl` holds the labels of `transforms/rows/cases.jsonl`. `replay/audit.jsonl` grades the replay of `transforms/rows/recording` described in `specification/audit.md`.

| Output | Command line (from this folder) | Test |
| --- | --- | --- |
| `golden/audit-decide.jsonl` | `audit small/decide.jsonl small/decide-key.jsonl` | `audit::old_goldens_hold` |
| `golden/audit-decide-0.4.jsonl` | the same with `--threshold 0.4` | `audit::old_goldens_hold` |
| `golden/audit-decide-band.jsonl` | `audit small/decide-band.jsonl small/decide-key.jsonl` | `audit::old_goldens_hold` |
| `golden/audit-decide-bare.jsonl` | `audit small/decide-bare.jsonl small/decide-key.jsonl` | `audit::old_goldens_hold` |
| `golden/audit-choose.jsonl` | `audit small/choose.jsonl small/choose-key.jsonl` | `audit::old_goldens_hold` |
| `golden/audit-annotate.jsonl` | `audit small/annotate.jsonl small/annotate-key.jsonl` | `audit::old_goldens_hold` |
| `golden/audit-249.jsonl` | `audit 249/control.jsonl 249/key.jsonl --by verb` | `audit::old_goldens_hold` |
| `golden/audit-249-seed.jsonl` | `audit 249/control.jsonl 249/key-noparts.jsonl --by verb --seed 249` | `audit::old_goldens_hold` |
| `golden/extra/audit-decide-reversed.jsonl` | `audit small/decide-reversed.jsonl small/decide-key-noparts.jsonl` | `audit::old_goldens_hold` |
| `golden/extra/audit-decide-odd.jsonl` | `audit small/decide.jsonl small/decide-key-odd.jsonl` | `audit::old_goldens_hold` |
| `golden/extra/audit-choose-target-1.jsonl` | `audit small/choose.jsonl small/choose-key.jsonl --target 1` | `audit::old_goldens_hold` |
| `golden/extra/audit-249-question-seed-7.jsonl` | `audit 249/control.jsonl 249/key.jsonl --by question --seed 7` | `audit::old_goldens_hold` |
| `golden/table/audit-decide.txt` | `audit small/decide.jsonl small/decide-key.jsonl --table` | `audit::old_goldens_hold` |
| `golden/table/audit-choose.txt` | `audit small/choose.jsonl small/choose-key.jsonl --table` | `audit::old_goldens_hold` |
| `golden/table/audit-annotate.txt` | `audit small/annotate.jsonl small/annotate-key.jsonl --table` | `audit::old_goldens_hold` |
| `golden/diff-decide-cuts.jsonl` | `diff small/decide.jsonl --key small/decide-key.jsonl --compare-threshold 0.4` | `diff::goldens_match` |
| `golden/diff-decide-wordings.jsonl` | `diff small/decide.jsonl small/decide-b.jsonl --key small/decide-key.jsonl` | `diff::goldens_match` |
| `golden/diff-decide-nokey.jsonl` | `diff small/decide.jsonl small/decide-b.jsonl` | `diff::goldens_match` |
| `golden/diff-choose.jsonl` | `diff small/choose.jsonl small/choose-b.jsonl --key small/choose-key.jsonl` | `diff::goldens_match` |
| `golden/diff-249-cuts.jsonl` | `diff 249/control.jsonl --key 249/key.jsonl --compare-threshold 0.42` | `diff::goldens_match` |
| `golden/diff-249-soft.jsonl` | `diff 249/control.jsonl 249/soft.jsonl --key 249/key.jsonl` | `diff::goldens_match` |
| `golden/extra/diff-annotate.jsonl` | `diff small/annotate.jsonl --key small/annotate-key.jsonl --compare-threshold 0.75` | `diff::goldens_match` |
| `golden/extra/diff-249-cuts-nokey.jsonl` | `diff 249/control.jsonl --compare-threshold 0.42` | `diff::goldens_match` |
| `golden/table/diff-decide-cuts.txt` | `diff small/decide.jsonl --key small/decide-key.jsonl --compare-threshold 0.4 --table` | `diff::tables_match_byte_for_byte` |
| `golden/table/diff-decide-nokey.txt` | `diff small/decide.jsonl small/decide-b.jsonl --table` | `diff::tables_match_byte_for_byte` |
| `golden/table/diff-choose.txt` | `diff small/choose.jsonl small/choose-b.jsonl --key small/choose-key.jsonl --table` | `diff::tables_match_byte_for_byte` |

Ticket 0125 added members and table lines beside the old ones. `audit::old_goldens_hold` removes `precision`, `f1`, `r_precision`, `mean_level_distance`, and `steady`, and the table lines those members print, before it compares. Every other byte must match.

### Ticket 0125 fixtures

`abbey/rows.jsonl` holds 70 `decide --details` lines saved by a benchmark's run over song titles, one question: is the title a Beatles song on Abbey Road? `abbey/key.jsonl` is that benchmark's key, by title. `abbey/key-tune.jsonl` is the same key with `"part": "tune"` on every line, from `jq -c '. + {part: "tune"}'`. Graded with `--id /input`.

`better/` holds hand fixtures for `steady.better`. Each has eight records and a key with no parts, so audit reads twenty seeded splits of four held records each. The held parts at seed 0 hold these records, split 0 first:

| Split | Held records | Split | Held records |
| ---: | --- | ---: | --- |
| 0 | 1, 4, 5, 8 | 10 | 1, 5, 6, 8 |
| 1 | 3, 5, 6, 7 | 11 | 2, 3, 4, 8 |
| 2 | 4, 5, 6, 7 | 12 | 2, 5, 7, 8 |
| 3 | 1, 4, 5, 8 | 13 | 2, 3, 4, 7 |
| 4 | 3, 4, 6, 7 | 14 | 1, 4, 7, 8 |
| 5 | 1, 2, 4, 6 | 15 | 2, 4, 5, 6 |
| 6 | 1, 4, 6, 8 | 16 | 1, 3, 6, 8 |
| 7 | 1, 3, 4, 6 | 17 | 2, 3, 5, 6 |
| 8 | 3, 5, 7, 8 | 18 | 1, 3, 4, 5 |
| 9 | 2, 4, 6, 8 | 19 | 1, 3, 6, 7 |

- `better/decide.jsonl`: records 1 to 4 are keyed yes at p 0.9, 0.8, 0.75, and 0.7. Records 5 to 8 are keyed no at p 0.6, 0.55, 0.3, and 0.2. The run's rule is 0.5, which calls 5 and 6 yes. A tuning part holding record 5 tunes 0.61. One holding 6 and not 5 tunes 0.56. One holding neither tunes 0.5. The splits tune 0.61 ten times, so `steady.cut` is 0.61. It beats 0.5 on every held part that holds 5 or 6, and ties on the rest: splits 11, 13, and 14. `better` is 17. Counting each split's own bar instead gives 7.
- `better/tie.jsonl`: yes at p 0.9, 0.85, 0.8, and 0.7, and no at 0.3, 0.2, 0.15, and 0.1. Every split tunes 0.5, the run's own rule, so every held part ties and `better` is 0. Counting a tie as a win gives 20.
- `better/choose-reach.jsonl`, run as printed with no threshold, `--target 0.9`: records 1 to 4, 7, and 8 pick the keyed option at top 0.95, 0.9, 0.85, 0.8, 0.7, and 0.65. Records 5 and 6 pick wrong at top 0.6 and 0.55. `steady.cut` is 0.61 again. A held part holding 5 or 6 sits below the target as run and at 1.0 at 0.61, so the cut wins by the target. The other three held parts reach the target both ways with the same right answers, so the cut does not win. `better` is 17.
- `better/choose-more.jsonl`, saved under a threshold of 0.9, `--target 0.8`: every pick is right. Records 1, 2, and 7 printed their pick at top 0.95, 0.92, and 0.99. The other five printed null at top 0.85, 0.8, 0.75, 0.7, and 0.6. Every split tunes 0.01. Every held part holds a null record, so 0.01 answers more at equal agreement 1.0 and wins. `better` is 20. Comparing agreement alone gives 0.

`verbs/` holds one hand fixture per verb for `audit_verbs::each_verb_grades`.

- `tag.jsonl`: labels `x` and `y`, six records. The key lists the labels that apply, and `t6` is null. Label `x`: said yes on t1, t2, t5 and no on t3, t4; the key says yes on t1, t2, t4, t5. That is 4 right, one false no, precision 3/3, recall 3/4, f1 6/7. Label `y`: said yes on t2, t3, t5; the key says yes on t3, t5. That is 4 right, one false yes, precision 2/3, recall 2/2, f1 4/5. The pooled row holds 12 answers, 10 labeled, 8 right, precision 5/6, recall 5/6, f1 10/12.
- `score.jsonl`: levels low, mid, high, every key line in the tuning part. At the midpoint cuts 0.5 and 1.5, s2 (0.7, keyed low) and s5 (1.6, keyed mid) are wrong: 4 right, mean level distance 2/6. The first cut rises to 0.71, the nearest place above 0.7 and at most 0.9. The second rises to 1.61, the nearest place above 1.6 and at most 1.9. Then all six are right, and no move gets more.
- `rank.jsonl`: five questions, one per edge row of the ticket's R-precision table. R counts the labeled relevant rows present: 2 (rows at p 0.9 and 0.7, with 0.8 between: 0.5), 1 (`m4` to `m7` are keyed yes but absent: 1.0), 0 (null), 0 (null), and 2 with a tie at p 0.7, where the earlier `p2`, keyed no, takes the second place: 0.5.
- `find.jsonl`: four lines with no `input`, so the ids are the line numbers. Line 1 picks `u002`, right. Line 2 prints null with `none` on top, right. Line 3 prints null with `u001` and `none` tied, so it is tied. Line 4 picks `u003` where the key says `u001`, wrong.
- `annotate.jsonl`: three records with a `decide`, a `choose`, and a `score` member. `urgent`: a1 right, a2 a false no, a3 a false yes. `effort`: levels small and large at the cut 0.5, so a3 at 0.6 reads large where the key says small.

`write/` holds the question files and keys for `audit_write`. `decide.json` is the payment question of `transforms/rows` with CRLF line ends, an escaped `threshold` key, and a threshold of 0.9. `key.jsonl` is `replay/key.jsonl` with `C-12` keyed no and a part on every line: `C-15` and `C-29` held, the rest tuned. The tuning part holds `C-12` keyed no at p 0.58 and `C-39` keyed yes at p 0.81, and every other record sits outside that range, so the bar is the lowest cut above 0.58: 0.59. On the held part, 0.59 gets `C-15` (no, p 0.51) and `C-29` (yes, p 0.79) right. Both 0.5 and 0.9 get one of them wrong. `set.json` is `demos/14-grade-a-batch/checks.json` with its threshold set to 0.95. `set-key.jsonl` keys only `correct`, from the cases' `human_correct`, with `E-05` held. The tuning part puts `correct` at p 0.98, 0.62, and 0.94 for yes and 0.02, 0.02, and 0.53 for no, so the bar is 0.54. It gets `E-05` (yes, p 0.94) right where 0.95 does not. `score.json` and `rank.json` hold the questions of demos 17 and 06.

`audit::every_fixture_keeps_its_checksum` fails on a missing, extra, or changed file.

| File | SHA-256 |
| --- | --- |
| `249/control.jsonl` | `c9abe7ba38243fe5fbf577a94754067de873be4b0b00339ec31eed40138d8bdb` |
| `249/key-noparts.jsonl` | `b6386622b98cf36523623ab22516629bfe38742e35f7f4d96f2db8cd87bd387b` |
| `249/key.jsonl` | `8e5e8fb7c799b0c64d1502167fc8f010d4a598202b060b0fd407600b68429b59` |
| `249/soft.jsonl` | `12dd593528336582c930e9f4472708018fd5bd67d9f1a3337e613773e734a08b` |
| `abbey/key-tune.jsonl` | `489151528893b560763f9abca1cc0d81a735e1a63310de444d19295b94f7f9e7` |
| `abbey/key.jsonl` | `3b12e168ca9715a775bf3bc759c20f6e018f7d1d1763a8c6352c59d2a42c242e` |
| `abbey/rows.jsonl` | `70b54a7636132e4576538107d45b2ff1841709de3eceb2a8b5970404b2678952` |
| `better/choose-more-key.jsonl` | `4400e79ce28f6f352d9ec7db606df7647879ec40aee49c52e210013be5776d99` |
| `better/choose-more.jsonl` | `9bd9459368a823269e6de6057f9228f212fbfd4ad066eecc52ce96f5482d692f` |
| `better/choose-reach-key.jsonl` | `641ab843cc9a72c3d8adca4e77e1da3393d40853dd2ca6c3f4270d921febb15e` |
| `better/choose-reach.jsonl` | `cfdc382795a8d25dfad725f6c0e3e1cd6691b1a684d1b2353fbd7f67162a37e6` |
| `better/decide-key.jsonl` | `8b47ff1106e77c8fd4c00802006c2fe9ca449e4e9e94b00e65bf6b04ad8898b0` |
| `better/decide.jsonl` | `07082f6c1c542f5e70f21e579df3f824280c4dd59e8b015f439e1e72b41aedd8` |
| `better/tie-key.jsonl` | `8b47ff1106e77c8fd4c00802006c2fe9ca449e4e9e94b00e65bf6b04ad8898b0` |
| `better/tie.jsonl` | `d74c6d3b93823583fc02fcd2b9ba3be5ef11a98e7ab5bfa32c495d492cdf0386` |
| `golden/audit-249-seed.jsonl` | `2e1ee2e57e9ba1fdace275a7aee4848d810e9d432feb3ac5ba1106d2780a8af8` |
| `golden/audit-249.jsonl` | `d7521a7c81976318f9e49d038a2dee035b77d8caf126513d645ca7ee31348e6f` |
| `golden/audit-annotate.jsonl` | `7e4e080af466ee8c1504ad01a54b3a46c4630056ebbd80de7ae04e344fcc10d2` |
| `golden/audit-choose.jsonl` | `55b7e1aa01c8f7ac5d8c2c3252391033ab6056eb29519c0c5bf2034580aa59be` |
| `golden/audit-decide-0.4.jsonl` | `1df25db52752f6591ec74c052609e696a368a0bd729b2ea98296bf1da2d14593` |
| `golden/audit-decide-band.jsonl` | `95a814d04e51515256e305f61a10858c3f0371a39127aac3ebd554b29e01efdc` |
| `golden/audit-decide-bare.jsonl` | `bd64e19970c3cafcb5f301810f9256cf7da941beebcd32b9896b70f310134de8` |
| `golden/audit-decide.jsonl` | `7f14cfb18393671db70eb08a8cdc8ed31bbe7bc14d55c89bf02fb2a90339f8dd` |
| `golden/diff-249-cuts.jsonl` | `76dfffe53438500267a548456deea5fdfc454ab05fec0a0e284ba52208475dd3` |
| `golden/diff-249-soft.jsonl` | `69b1c96922b4c0c5bdc40ab919e0023eb08bac4b10baafd20d3af61b5f53ea44` |
| `golden/diff-choose.jsonl` | `846ba7f3d93938f89c403eec7b973aed034b3d20cadc06970a4216e2e8b58d09` |
| `golden/diff-decide-cuts.jsonl` | `ae00a2514257d35c36658c3c63abe2c92e9dbc711950bdcef7864f891d081682` |
| `golden/diff-decide-nokey.jsonl` | `ad99f8572fa59ab30168859f119114892656e0e61efbe0e633cba5c93727a284` |
| `golden/diff-decide-wordings.jsonl` | `593398f0617796069712dcbad935a0b181ed57e1a78c362e41fcaf15c0ccc3f5` |
| `golden/extra/audit-249-question-seed-7.jsonl` | `c65d036e65d60803ef4da3ae87c3037ddb4ef8ea3853d6ba85f04600aabb05bb` |
| `golden/extra/audit-choose-target-1.jsonl` | `1a7c2316601e6d5ce2a6b92c2e1c4d16ce56432ecda3e34fbda7ef284a893cda` |
| `golden/extra/audit-decide-odd.jsonl` | `f4cf6493b4e0620bedff38628ade3b48e1c2fba91c35934c5a6d9ad6e4a930ac` |
| `golden/extra/audit-decide-reversed.jsonl` | `376ce4dbd96c6d1619ee7ff340e54510785c528a2f5560ef83c027a726704282` |
| `golden/extra/diff-249-cuts-nokey.jsonl` | `1fc7ba84a3a666004872ea0e619c5451f64daa67a6bda1aefa9540a398a44458` |
| `golden/extra/diff-annotate.jsonl` | `ab7ac6f3064bfea00a0ca9d075183f1f1c33947e507584c9206a27f52d51514a` |
| `golden/table/audit-annotate.txt` | `64c8acad1ccf1dafe3aad01fb5e0cf8f96e7dbde83a9769e2d2bde767713822a` |
| `golden/table/audit-choose.txt` | `acc4564c3a8e66acad7734311e66810c586f8444a86b67b3a33fcfd179d890c0` |
| `golden/table/audit-decide.txt` | `6a79b6e1a273539233a5b61df3b6ff4a9bd354504d347a79d05ff5b0beaf2513` |
| `golden/table/diff-choose.txt` | `a24e14564533ff91f6f038096df9e4d6fafa72ac3555f7177deefa4182868b6d` |
| `golden/table/diff-decide-cuts.txt` | `c56e94bc2e3c09c525e7ef8fc3e2190a5af4216c5d91d769c4176cfea06536c8` |
| `golden/table/diff-decide-nokey.txt` | `82edef392a2ddee9528a931453abd42c57d99a5c67a98e2984ffcc6e8be9f9c1` |
| `replay/audit.jsonl` | `54901b07eec7e0569ff8492deb1822058d62e61b4830e0a26bc78c7a68b4d953` |
| `replay/key.jsonl` | `6469595eef17159ed9563de7b84bdc4396249fb2bb8a704397e5cc31c0619042` |
| `small/annotate-key.jsonl` | `fcaebb1e268c468d697e6bc6852c4b5aab5198e544552711d7d5c1b27e523b26` |
| `small/annotate.jsonl` | `fd1fa69b635109d9a243d3ad43ec94fbac645c627967336ce3ed5265e72b42c3` |
| `small/choose-b.jsonl` | `b227bc45a7fbcb64db89352bda57d0da5eb53a9b9df01f2332c956dc844d8d51` |
| `small/choose-key.jsonl` | `f908ef59696a2fc7ba06001e3bbaaf53e52427a4adf5d8f2d347e8bb7ca0ddfa` |
| `small/choose.jsonl` | `b8f2178160860be3a58b4801e72eb68ded2cdfc0fd1675c34a23e95f8e9be7cb` |
| `small/decide-b.jsonl` | `a63d035f5a867b8080cca87efba5aeaec0ecbf3b53ffb6339f7c80ca1b0f785f` |
| `small/decide-band.jsonl` | `c8dc93088bc80ffbefa5087c5639a48da1eb5a10408ec760c4547e68674aa2a6` |
| `small/decide-bare.jsonl` | `b9f53267f0fb1fa0d89250f42c3c6f2bbd4c975548dde94668b55b545dad2e11` |
| `small/decide-key-noparts.jsonl` | `ebd896c265f68c7a4e71387ab8af8b495ae5f9406d9248b4cb2f9fadd87d0e25` |
| `small/decide-key-odd.jsonl` | `5aea71cae954171b2003947a27294ab86c8e17a746124cb04b0d9b5100d96128` |
| `small/decide-key.jsonl` | `10461a043dd3aa8feb8b5647212abd31f779511b274df5ddd2e9269403b59df4` |
| `small/decide-reversed.jsonl` | `e51b0c73bd7183715e3cda4bf71a02e0a0f4ca22cdc051ecedf83430a497ea86` |
| `small/decide.jsonl` | `fd6546d99ca631ec96569e432a8ec6d40889c312a445728be9a678196b0a06c6` |
| `verbs/annotate-key.jsonl` | `d39de81ae6d31ba4f2dc748c259d689a0e9ccb538d744f3a4221d1403454b63f` |
| `verbs/annotate.jsonl` | `6ad41b300b9de9c2fc680b32da7627ee6f5d7bb0c9eaa7fe17d7e641a5ca8704` |
| `verbs/find-key.jsonl` | `f095553f48b4ef6189015071b3f8e74185d71a7ae4c4aa399e7173a425cf6562` |
| `verbs/find.jsonl` | `bb64e67ef52bd4f3aafaa22f7ea102209145e7585fd668db7b62e67c3d373ffe` |
| `verbs/rank-key.jsonl` | `c45b0db08b0b382c6f28c96a35f76f2a9b600ddae3009cc50910f519854cb8f4` |
| `verbs/rank.jsonl` | `a47981836cdd181ae6905a1adac191c63adf90d6936e43d54e6c3b402dd2d64d` |
| `verbs/score-key.jsonl` | `be01ccf4a2ec3478c4628cce5a07f73b263251497db8c467d585d1ded7b92c37` |
| `verbs/score.jsonl` | `fb70310ebc6d57166fc88c2701be55b902e0c3cb96a6bed9c66fa38e909e8f35` |
| `verbs/tag-key.jsonl` | `df891f08ee40f159c81b5172dfe91cf2a19189422b510fd8fdb5e19245e217a8` |
| `verbs/tag.jsonl` | `789fef009c2dd692d1444eaf824b680189665d1f064d05bc4ec2060f54ef4c32` |
| `write/decide.json` | `9f95c830aa63f2e905f530067cb8139af4d334787cb9ddeb9b30ac73c96ea683` |
| `write/key.jsonl` | `2208a2f79f308ff14ceb303be29d8cf0dc44f7f84cac22ecddec152d25265864` |
| `write/rank.json` | `237516ad81b76d4cf9508a44d59c637f88a97ca5d5c5865567e16ba1e7d799d3` |
| `write/score.json` | `4388b91d7342267bfb7a1547d4c41e1276fb5667c0ecc999417e656f6793976a` |
| `write/set-key.jsonl` | `ade50a6f5cc0262d4b5b7c9d39fc2287d61825015a0318633d82683917f755dd` |
| `write/set.json` | `ba2b5e1c0d8c2f142cbdd19e744847eb5a18c9d2f5014ec96c7d22d5033c3979` |
