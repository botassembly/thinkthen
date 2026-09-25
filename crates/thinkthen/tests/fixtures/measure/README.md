# Measure fixtures

These files define `thinkthen audit` and `thinkthen diff`. The source is a prototype measurement script at commit `be7cea2e4aa41097e7f629e35b62dadedeaca544`. Its three files had these SHA-256 values at that commit:

| Prototype file | SHA-256 |
| --- | --- |
| `scripts/tools/measure.py` | `7be206812f0e41011a9b31c59030c480f8b6bed945d7976db452f3908f0316a1` |
| `tests/test_measure.py` | `d64f2d9088c4f6237c0d2a2a8faf06dc51f92cd4674dab20b2a4f7231c01fa4e` |
| `scripts/tools/README.md` | `fae695523a16d1f2921ee9c73358c36e1613b58ee6acc9d720c6bc6e1d9e7fc7` |

`small/`, `249/`, and the fourteen files directly under `golden/` are byte-for-byte copies of the prototype's `tests/fixtures/audit/` at that commit. One departs: Quick Fix qf-diff-warnings renamed the meta key in `small/annotate.jsonl` to `questions_sha256`, the key a real `annotate` line carries. The small fixtures share the placeholder digest `00`. The same Quick Fix recaptured `golden/diff-choose.jsonl` and `golden/table/diff-choose.txt` from `thinkthen diff` under the wider McNemar rule. Their McNemar p moved from 1.0 to 0.5. `249/` holds `thinkthen decide --details` lines replayed from an earlier experiment's recordings with every key unset: 272 yes/no questions about Beatles songs.

Three inputs are derived from `small/`. `decide-reversed.jsonl` holds the lines of `decide.jsonl` in reverse. `decide-key-noparts.jsonl` holds the key without `part`. `decide-key-odd.jsonl` holds the key without `part` and without `r6`.

The files under `golden/extra/`, `golden/table/`, and `replay/audit.jsonl` were captured from the prototype script at that commit, read with `git show` into a scratch file and run under Python 3.12.3 from this folder. `replay/key.jsonl` holds the labels of `transforms/rows/cases.jsonl`. `replay/audit.jsonl` grades the replay of `transforms/rows/recording` described in `specification/audit.md`.

| Output | Command line (from this folder) | Test |
| --- | --- | --- |
| `golden/audit-decide.jsonl` | `audit small/decide.jsonl small/decide-key.jsonl` | `audit::goldens_match` |
| `golden/audit-decide-0.4.jsonl` | the same with `--threshold 0.4` | `audit::goldens_match` |
| `golden/audit-decide-band.jsonl` | `audit small/decide-band.jsonl small/decide-key.jsonl` | `audit::goldens_match` |
| `golden/audit-decide-bare.jsonl` | `audit small/decide-bare.jsonl small/decide-key.jsonl` | `audit::goldens_match` |
| `golden/audit-choose.jsonl` | `audit small/choose.jsonl small/choose-key.jsonl` | `audit::goldens_match` |
| `golden/audit-annotate.jsonl` | `audit small/annotate.jsonl small/annotate-key.jsonl` | `audit::goldens_match` |
| `golden/audit-249.jsonl` | `audit 249/control.jsonl 249/key.jsonl --by verb` | `audit::goldens_match` |
| `golden/audit-249-seed.jsonl` | `audit 249/control.jsonl 249/key-noparts.jsonl --by verb --seed 249` | `audit::goldens_match` |
| `golden/extra/audit-decide-reversed.jsonl` | `audit small/decide-reversed.jsonl small/decide-key-noparts.jsonl` | `audit::goldens_match` |
| `golden/extra/audit-decide-odd.jsonl` | `audit small/decide.jsonl small/decide-key-odd.jsonl` | `audit::goldens_match` |
| `golden/extra/audit-choose-target-1.jsonl` | `audit small/choose.jsonl small/choose-key.jsonl --target 1` | `audit::goldens_match` |
| `golden/extra/audit-249-question-seed-7.jsonl` | `audit 249/control.jsonl 249/key.jsonl --by question --seed 7` | `audit::goldens_match` |
| `golden/table/audit-decide.txt` | `audit small/decide.jsonl small/decide-key.jsonl --table` | `audit::tables_match_byte_for_byte` |
| `golden/table/audit-choose.txt` | `audit small/choose.jsonl small/choose-key.jsonl --table` | `audit::tables_match_byte_for_byte` |
| `golden/table/audit-annotate.txt` | `audit small/annotate.jsonl small/annotate-key.jsonl --table` | `audit::tables_match_byte_for_byte` |
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

`audit::every_fixture_keeps_its_checksum` fails on a missing, extra, or changed file.

| File | SHA-256 |
| --- | --- |
| `249/control.jsonl` | `c9abe7ba38243fe5fbf577a94754067de873be4b0b00339ec31eed40138d8bdb` |
| `249/key-noparts.jsonl` | `b6386622b98cf36523623ab22516629bfe38742e35f7f4d96f2db8cd87bd387b` |
| `249/key.jsonl` | `8e5e8fb7c799b0c64d1502167fc8f010d4a598202b060b0fd407600b68429b59` |
| `249/soft.jsonl` | `12dd593528336582c930e9f4472708018fd5bd67d9f1a3337e613773e734a08b` |
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
| `golden/diff-choose.jsonl` | `fd677ab15a786ceaae633b89ddb46badbc36a953f721ef0d4d16e13c1600e44f` |
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
| `golden/table/diff-choose.txt` | `7f1a11677eba121025ab46b1ad5ffdd6ec09e3cb4ac0d0519960b5ba7301c186` |
| `golden/table/diff-decide-cuts.txt` | `c56e94bc2e3c09c525e7ef8fc3e2190a5af4216c5d91d769c4176cfea06536c8` |
| `golden/table/diff-decide-nokey.txt` | `82edef392a2ddee9528a931453abd42c57d99a5c67a98e2984ffcc6e8be9f9c1` |
| `replay/audit.jsonl` | `54901b07eec7e0569ff8492deb1822058d62e61b4830e0a26bc78c7a68b4d953` |
| `replay/key.jsonl` | `6469595eef17159ed9563de7b84bdc4396249fb2bb8a704397e5cc31c0619042` |
| `small/annotate-key.jsonl` | `fcaebb1e268c468d697e6bc6852c4b5aab5198e544552711d7d5c1b27e523b26` |
| `small/annotate.jsonl` | `c827c6ca44198b10c0561b49d930d85f74bdba88549169e1cc187e2320c24270` |
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
