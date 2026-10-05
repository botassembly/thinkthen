# 0380: Windows command and Rust crate

Slices A/B/C landed through `6d26206aa`: Windows x86-64 command ZIP/release routing, the PowerShell development installer, and native configuration/usage privacy plus console interruption. Portable/Linux checks, Unix installer regressions and documentation replay passed; they do not establish Windows execution.

The installer bounds downloads, validates archive/checksum/PE identity, uses owner-only NTFS creation and rejects foreign ownership or reparse paths without repair. It serializes replacement, preserves the primary error and retains recovery snapshots after failed rollback; it does not promise a two-file atomic transaction or power-loss durability. It changes no profile, registry, execution policy or runtime configuration. PowerShell 7 on Linux exercised actual function/transaction ASTs; prepared PowerShell 5.1/7 native cases remain unexecuted.

Native runtime ownership is confined to the admitted Windows FFI leaves. Usage permits only the token user and LocalSystem, checks protected descriptors/handle identity and keeps read-only configuration warning/refusal behavior distinct. Interruption retains cancellation/cleanup ordering. Accepted design choices are preserved here because product code points to no separate design file; Ian can overturn them.

The combined real Windows run is authorized. Required native permissions, replacement, console cancellation and installed-package behavior remain release criteria. Linux/portable results prove neither native ACL/NTFS behavior nor console event handling; unsigned development output does not imply public release approval.

The first rehearsal reached Windows compilation and failed target-specific Clippy checks. The reviewed correction retains acknowledgment/write ordering and SID/ACL alignment checks, loads unit-test helpers once, scopes fixture assertions to tests and keeps the Unix forbidden-API list valid on Windows. A fresh High source review accepted it. Linux policy, formatting, Clippy and focused cancellation/configuration tests passed; the native run must still qualify it.

The next native run cleared dependency preparation but found production panic lints still applied to the separate Windows integration-test crate. Two reasoned, test-only allowances now cover its existing assertions; production lint restrictions and cleanup behavior are unchanged. The source total grew by exactly two lines. One fresh review accepted the correction, with native lint and runtime qualification still required.
