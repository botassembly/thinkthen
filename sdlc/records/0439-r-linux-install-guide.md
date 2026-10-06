# 0439: Linux R installation guide

The guide states Linux package prerequisites and binary host limits, corrects the ARM64 repository URL, explains native loading, and separates published 0.1.2 from development builds. The installed public consumer checks a saved-answer example.

Fresh review found that the replay consumer had lost its counted loopback address. The fix reuses the seeded answer and checks replay hits and misses through the same installed call. Focused install, R smoke, site links and public-text checks passed. Full tests and lint run on the landing commit with an owned empty configuration.

## What the build taught us

A reported zero-send count cannot replace the counted server. Keep the replay consumer on the same request identity and check the miss path as well.
