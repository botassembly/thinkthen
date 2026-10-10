# 0384: Bundle the Windows native library in NuGet

Final NuGet assembly now reads every native RID from the package inventory and takes its bytes from the corresponding checksummed C archive. Windows uses bin/thinkthen.dll. Managed assemblies, nuspec and documentation bytes remain intact; native content types cover the added macOS libraries.

The packaging assessment found that the old registry step copied a Linux-only NuGet package unchanged. The correction lands through 0530 at f415310b07a536a24cb4c90c5f98c8f6a0ac5bf7. Fresh review accepts 8bf850435a4efee744ad27fcbde3983a16cc5474. All 40 existing bounded registry cases pass, including exact bytes for all five RIDs, missing Windows archive and missing DLL refusals. Repeated assembly produces identical bytes. Existing Linux installed managed consumers pass.

The 2026-10-10 ruling closes reviewed packaging without waiting on Windows execution. Actual final-package installation and the counted Windows C# loopback call remain with 0530 and the candidate. Native Windows execution is not claimed. Publishing remains on Ian's go.

## What the build taught us

A correct platform loader cannot load a native library omitted from its package. Inspect the artifact users install, and source each bundled RID from the inventoried platform build instead of copying one host's package unchanged.
