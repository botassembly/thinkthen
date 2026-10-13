# Installed coverage baseline at da1e3bb17

Measured source: `da1e3bb17e98ae4cde5ee75573351d36f4e3ae4d`. Both Linux runs completed 31 collections; Objective-C Foundation was omitted because it requires macOS. The first took 3611.8 seconds; the second took 3768.8 seconds. Each measured 59,962 mapped nonblank lines across 458 production files.

The [0560 PM amendment](../../tickets/0560-measure-coverage-on-every-surface.md#evidence) accepts this pair as the 0.2 baseline and waives reproducibility. Each candidate collection must reach at least its lower baseline count below, minus lines deleted by the cleanup waves. Objective-C joins at the candidate on macOS. No full coverage measurement runs before the candidate.

| Collection | First reached | Second reached | Lower baseline count |
| --- | ---: | ---: | ---: |
| c-source-unit | 16654 | 16653 | 16653 |
| cli-source | 15153 | 15155 | 15153 |
| installed-ada | 20485 | 20485 | 20485 |
| installed-c | 21927 | 21928 | 21927 |
| installed-cobol | 20605 | 20605 | 20605 |
| installed-cpp | 16994 | 16994 | 16994 |
| installed-csharp | 18081 | 18075 | 18075 |
| installed-dart | 15826 | 15828 | 15826 |
| installed-flutter | 15706 | 15706 | 15706 |
| installed-go | 16723 | 16722 | 16722 |
| installed-java | 14722 | 14722 | 14722 |
| installed-java-usage | 6922 | 6922 | 6922 |
| installed-javascript | 15020 | 15020 | 15020 |
| installed-kotlin | 7520 | 7519 | 7519 |
| installed-kotlin-usage | 6922 | 6922 | 6922 |
| installed-pandas | 16489 | 16489 | 16489 |
| installed-php | 16095 | 16095 | 16095 |
| installed-polars | 13777 | 13778 | 13777 |
| installed-python | 16489 | 16489 | 16489 |
| installed-python-polars | 16489 | 16488 | 16488 |
| installed-python-shared | 16112 | 16109 | 16109 |
| installed-r | 18113 | 18113 | 18113 |
| installed-ruby | 17167 | 17168 | 17167 |
| installed-rust | 13229 | 13228 | 13228 |
| installed-scala | 8091 | 8090 | 8090 |
| installed-scala-usage | 6922 | 6922 | 6922 |
| installed-swift | 16701 | 16702 | 16701 |
| installed-typescript | 15020 | 15020 | 15020 |
| installed-zig | 22335 | 22335 | 22335 |
| rust-integration | 40242 | 40240 | 40240 |
| source-unit | 22264 | 22263 | 22263 |

| Total | First | Second |
| --- | ---: | ---: |
| mapped_nonblank | 59962 | 59962 |
| observed_unit_only | 3417 | 3422 |
| unobserved | 9624 | 9623 |
