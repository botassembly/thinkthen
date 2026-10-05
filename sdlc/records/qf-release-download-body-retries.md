# Retry incomplete release SDK downloads

Final hosted rehearsal `37386703492`, Linux x86-64 job `112021669257`, stopped on `85ed73230588380ec7d81c08b58ad91074269ee2` when the .NET archive differed from its pinned SHA-512. The failed archive and response headers were not retained. The exact hosted cause remains unconfirmed. A controlled response with early EOF reproduces the same checksum refusal on the original downloader: it accepts 26 bytes despite a Content-Length of 213571282 and makes one request.

Microsoft's [8.0 release metadata](https://builds.dotnet.microsoft.com/dotnet/release-metadata/8.0/releases.json) and [8.0.31 checksums](https://builds.dotnet.microsoft.com/dotnet/checksums/8.0.31-sha.txt), linked by its [SDK release notes](https://github.com/dotnet/core/blob/main/release-notes/8.0/8.0.31/8.0.131.md), confirm the existing 8.0.131 Linux x64 URL and SHA-512. Both pins stay unchanged. The checksum is `a6182d3f136c524485da248156b263012eba70179a0849131b5331548770fc149a42347f7865fdbd4f81057b28fa68c2395ab0e9549f2c8a268310f48a2304eb`.

The downloader now covers body reads within its existing three attempts and 10/20 second pauses. It retries early EOF against Content-Length, partial HTTP responses, body timeouts, connection resets and incomplete reads. It removes its partial output before retrying or reporting the final failure. A complete response still goes through the unchanged checksum check before extraction. Acquisition flags, URL selection, installed selectors and extraction rules stay unchanged.

The existing language-tools fixture passes. Its body table fails on the original code at the short-response case. Each case now recovers on the third complete response and stops after three bad responses without retaining a partial archive. Existing changed-checksum refusal still prevents extraction. Workflow checks and Python syntax pass. `git diff --check` passes. The coordinator owns full tests and lint on the reviewed landing commit.

A fresh real download through the corrected `sdk_path` on 2026-10-05 returned HTTP 200, Content-Length 213571282, ETag `0x8DF0DEE907AD9AD` and Last-Modified 2026-09-08. All 213571282 bytes matched the unchanged pin. The existing extraction and executable probe returned SDK version `8.0.131`. This used a job-owned scratch directory and lane lock under a user service with MemoryMax 12G and MemorySwapMax 1G. It changed no global toolchain. This local acquisition qualifies the downloader path; the coordinator must rerun the authorized hosted rehearsal to establish runner qualification.

## What the build taught us

Opening a response successfully does not establish that its body arrived completely. Sized urllib reads can end early without an exception. Retry those transient transport failures within the existing bound, and retain the independent checksum refusal for different complete content.
