# Windows static-library distribution waits for stage 3

Status: open. Ticket 0381 permits DLL-only distribution for Windows 0.2. Ian's core-scope ruling keeps CLI/Rust, C DLL and Python in this release.
Milestone: later

Stage 3 must decide how to hide Rust internal symbols in a Windows COFF static library, prove static-link coexistence and define its metadata before distributing it. The Windows 0.2 archive contains the header, public DLL and MSVC import library. No native localization experiment ran or failed. Linux and macOS retain their existing static libraries and export rules.
