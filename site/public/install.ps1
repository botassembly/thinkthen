# Windows x86-64 development installer. Review this script before running it.
# Windows archives start with 0.2. Public signing and distribution remain open.
# Requires Windows PowerShell 5.1 or PowerShell 7 and a local NTFS directory.
[CmdletBinding()]
param([string] $Version = '')
Set-StrictMode -Version 2.0
$ErrorActionPreference = 'Stop'

function Convert-ReleaseVersion([string] $Value) {
    if ($Value -cnotmatch '\Av?[0-9]+\.[0-9]+\.[0-9]+\z') { throw 'Give a release version as X.Y.Z.' }
    $plain = $Value -creplace '^v', ''
    try { $parsed = [version] $plain } catch { throw 'Give a release version as X.Y.Z.' }
    if ($parsed -lt [version] '0.2.0') { throw 'Windows archives start with 0.2. Give a version of at least 0.2.0.' }
    return $plain
}

function Assert-Transport([uri] $Url) {
    if (-not $Url.IsAbsoluteUri -or $Url.UserInfo -or $Url.Fragment) { throw 'Refusing an unsafe download address.' }
    if ($Url.Scheme -eq 'https') { return }
    $ip = $null
    if ($Url.Scheme -eq 'http' -and [System.Net.IPAddress]::TryParse($Url.DnsSafeHost, [ref] $ip) -and [System.Net.IPAddress]::IsLoopback($ip)) { return }
    throw 'Downloads require HTTPS, or a numeric loopback address for offline fixtures.'
}

function Get-Base([string] $Value, [string] $Default) {
    if (-not $Value) { $Value = $Default }
    $url = [uri] $Value
    Assert-Transport $url
    if ($url.Query) { throw 'Download bases must have no query.' }
    return $url.AbsoluteUri.TrimEnd('/')
}

function Initialize-InstallHelpers {
    # Native directory creation supplies the access list at creation on both
    # .NET Framework and .NET. There is no initially public directory window.
    if ('ThinkThenInstall.Native' -as [type]) { return }
    Add-Type -TypeDefinition @'
using System;
using System.IO;
using System.Text;
using System.Diagnostics;
using System.ComponentModel;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
namespace ThinkThenInstall {
    public static class Native {
        [StructLayout(LayoutKind.Sequential)] struct SecurityAttributes {
            public int Length; public IntPtr Descriptor; public int Inherit;
        }
        [DllImport("advapi32.dll", CharSet=CharSet.Unicode, SetLastError=true)]
        static extern bool ConvertStringSecurityDescriptorToSecurityDescriptor(string text, uint revision, out IntPtr descriptor, out uint size);
        [DllImport("kernel32.dll", CharSet=CharSet.Unicode, SetLastError=true)]
        static extern bool CreateDirectory(string path, ref SecurityAttributes attributes);
        [DllImport("kernel32.dll")] static extern IntPtr LocalFree(IntPtr pointer);
        [DllImport("kernel32.dll")] static extern void GetNativeSystemInfo(IntPtr info);
        public static bool X64() {
            IntPtr info = Marshal.AllocHGlobal(64);
            try { GetNativeSystemInfo(info); return Marshal.ReadInt16(info) == 9; }
            finally { Marshal.FreeHGlobal(info); }
        }
        public static void PrivateDirectory(string path, string sid) {
            IntPtr descriptor; uint size;
            if (!ConvertStringSecurityDescriptorToSecurityDescriptor("O:" + sid + "D:P(A;OICI;FA;;;" + sid + ")(A;OICI;FA;;;SY)", 1, out descriptor, out size))
                throw new Win32Exception(Marshal.GetLastWin32Error());
            try {
                SecurityAttributes attrs = new SecurityAttributes();
                attrs.Length = Marshal.SizeOf(typeof(SecurityAttributes)); attrs.Descriptor = descriptor;
                if (!CreateDirectory(path, ref attrs)) throw new Win32Exception(Marshal.GetLastWin32Error());
            } finally { LocalFree(descriptor); }
        }
        [DllImport("kernel32.dll", EntryPoint="CreateFileW", CharSet=CharSet.Unicode, SetLastError=true)]
        static extern Microsoft.Win32.SafeHandles.SafeFileHandle CreateFile(string path, uint access, uint share, ref SecurityAttributes attributes, uint creation, uint flags, IntPtr template);
        public static FileStream NewFile(string path, string sid, bool readWrite) {
            IntPtr descriptor; uint size;
            if (!ConvertStringSecurityDescriptorToSecurityDescriptor("O:" + sid + "D:P(A;;FA;;;" + sid + ")(A;;FA;;;SY)", 1, out descriptor, out size))
                throw new Win32Exception(Marshal.GetLastWin32Error());
            try {
                SecurityAttributes attrs = new SecurityAttributes();
                attrs.Length = Marshal.SizeOf(typeof(SecurityAttributes)); attrs.Descriptor = descriptor;
                Microsoft.Win32.SafeHandles.SafeFileHandle handle = CreateFile(path, readWrite ? 0xc0000000u : 0x40000000u, 0, ref attrs, 1, 0x80, IntPtr.Zero);
                if (handle.IsInvalid) { int code = Marshal.GetLastWin32Error(); handle.Dispose(); throw new Win32Exception(code); }
                try { return new FileStream(handle, readWrite ? FileAccess.ReadWrite : FileAccess.Write, 65536, false); }
                catch { handle.Dispose(); throw; }
            } finally { LocalFree(descriptor); }
        }
        static async Task<string> Capture(StreamReader reader) {
            char[] buffer = new char[512]; StringBuilder text = new StringBuilder(); int count;
            while ((count = await reader.ReadAsync(buffer, 0, buffer.Length)) != 0) {
                if (text.Length + count > 4096) throw new IOException("Version output exceeds its limit.");
                text.Append(buffer, 0, count);
            }
            return text.ToString();
        }
        public static string Probe(string path) {
            ProcessStartInfo start = new ProcessStartInfo(path, "--version");
            start.UseShellExecute = false; start.CreateNoWindow = true;
            start.RedirectStandardOutput = true; start.RedirectStandardError = true;
            foreach (string key in new System.Collections.Generic.List<string>(start.EnvironmentVariables.Keys.CastStrings()))
                if (key.StartsWith("THINKTHEN_", StringComparison.OrdinalIgnoreCase) || key.EndsWith("_API_KEY", StringComparison.OrdinalIgnoreCase)) start.EnvironmentVariables.Remove(key);
            using (Process child = new Process()) {
                child.StartInfo = start;
                if (!child.Start()) throw new IOException("Could not launch staged command.");
                Task<string> output = Capture(child.StandardOutput), error = Capture(child.StandardError);
                Stopwatch clock = Stopwatch.StartNew();
                try {
                    while (!child.WaitForExit(25) || !output.IsCompleted || !error.IsCompleted) {
                        if (output.IsFaulted || error.IsFaulted) throw new IOException("Version output exceeds its limit.");
                        if (clock.ElapsedMilliseconds > 10000) throw new IOException("Version probe timed out.");
                    }
                    if (child.ExitCode != 0 || error.Result.Length != 0) throw new IOException("Staged command version probe failed.");
                    return output.Result;
                } finally {
                    if (!child.HasExited) { child.Kill(); child.WaitForExit(); }
                }
            }
        }
        static System.Collections.Generic.IEnumerable<string> CastStrings(this System.Collections.ICollection keys) {
            foreach (object key in keys) yield return (string)key;
        }
    }
}
'@
}

function Get-Download([uri] $Url, [long] $Limit) {
    # Disable automatic redirects: each destination must pass the same policy.
    for ($attempt = 0; $attempt -lt 3; $attempt++) {
        $current = $Url
        try {
            for ($redirect = 0; $redirect -le 5; $redirect++) {
                Assert-Transport $current
                $request = [System.Net.HttpWebRequest]::Create($current)
                $request.AllowAutoRedirect = $false
                $request.Timeout = 15000; $request.ReadWriteTimeout = 15000
                $request.Proxy = $null; $request.UseDefaultCredentials = $false
                $request.UserAgent = 'thinkthen-development-installer'
                $response = $null
                try {
                    $response = $request.GetResponse()
                    $status = [int] $response.StatusCode
                    if ($status -ge 300 -and $status -lt 400) {
                        if ($redirect -eq 5 -or -not $response.Headers['Location']) { throw 'Download redirect limit exceeded.' }
                        $next = [uri]::new($current, $response.Headers['Location'])
                        Assert-Transport $next
                        if ($current.Scheme -eq 'https' -and $next.Scheme -ne 'https') { throw 'Refusing a download transport downgrade.' }
                        $current = $next; continue
                    }
                    if ($status -ne 200) { throw 'Download did not return status 200.' }
                    if ($response.ContentLength -gt $Limit) { throw 'Download exceeds its byte limit.' }
                    $stream = $response.GetResponseStream(); $memory = [System.IO.MemoryStream]::new()
                    try {
                        $buffer = [byte[]]::new(65536)
                        $clock = [System.Diagnostics.Stopwatch]::StartNew()
                        while (($count = $stream.Read($buffer, 0, $buffer.Length)) -gt 0) {
                            if ($clock.Elapsed.TotalSeconds -gt 60) { throw 'Download exceeded its total timeout.' }
                            if ($memory.Length + $count -gt $Limit) { throw 'Download exceeds its byte limit.' }
                            $memory.Write($buffer, 0, $count)
                        }
                        if ($response.ContentLength -ge 0 -and $memory.Length -ne $response.ContentLength) { throw 'Download ended before its declared length.' }
                        return ,$memory.ToArray()
                    } finally { $stream.Dispose(); $memory.Dispose() }
                } finally { if ($response) { $response.Dispose() } }
            }
        } catch [System.Net.WebException] {
            $failure = $_.Exception
            $retry = $failure.Status -in @([System.Net.WebExceptionStatus]::ConnectFailure, [System.Net.WebExceptionStatus]::Timeout, [System.Net.WebExceptionStatus]::ConnectionClosed, [System.Net.WebExceptionStatus]::ReceiveFailure)
            if ($failure.Response) {
                $retry = [int] $failure.Response.StatusCode -in @(408, 429, 500, 502, 503, 504)
                $failure.Response.Dispose()
            }
            if (-not $retry -or $attempt -eq 2) { throw 'Could not download release data. Retry with an explicit -Version if latest selection failed.' }
        }
    }
    throw 'Download failed.'
}

function Get-LatestVersion([string] $Api) {
    $bytes = Get-Download ([uri] "$Api/repos/botassembly/thinkthen/releases") 1048576
    try { $releases = [System.Text.Encoding]::UTF8.GetString($bytes) | ConvertFrom-Json } catch { throw 'Release metadata is invalid. Give an explicit -Version.' }
    foreach ($release in $releases) {
        try {
            if ($release.draft -ne $false -or $release.prerelease -ne $false) { continue }
            $candidate = Convert-ReleaseVersion $release.tag_name
            $name = "thinkthen-$candidate-x86_64-pc-windows-msvc.zip"
            $assets = @($release.assets | ForEach-Object { $_.name })
            if ($assets -ccontains $name -and $assets -ccontains "$name.sha256") { return $candidate }
        } catch { continue }
    }
    throw 'No Windows release is available. Windows archives start with 0.2.'
}

function Get-BytesHash([byte[]] $Bytes) {
    $hash = [System.Security.Cryptography.SHA256]::Create()
    try { return ([System.BitConverter]::ToString($hash.ComputeHash($Bytes))).Replace('-', '').ToLowerInvariant() }
    finally { $hash.Dispose() }
}

function Assert-Checksum([byte[]] $Bytes, [byte[]] $Sidecar, [string] $Name) {
    $records = @([System.Text.Encoding]::UTF8.GetString($Sidecar) -split '\r?\n' | Where-Object { $_.Trim().Length -gt 0 })
    $pattern = '^([a-fA-F0-9]{64})(?:[ \t]+\*?' + [regex]::Escape($Name) + ')?[ \t]*$'
    if ($records.Count -ne 1 -or $records[0] -cnotmatch $pattern) { throw 'Checksum must contain exactly one valid record for this archive.' }
    if ((Get-BytesHash $Bytes) -ne $Matches[1].ToLowerInvariant()) { throw 'Archive checksum does not match.' }
}

function Assert-PE([byte[]] $Bytes) {
    if ($Bytes.Length -lt 64 -or $Bytes[0] -ne 77 -or $Bytes[1] -ne 90) { throw 'Command is not a Windows executable.' }
    $offset = [long] [System.BitConverter]::ToUInt32($Bytes, 60)
    if ($offset + 26 -gt $Bytes.Length) { throw 'Command has no complete PE header.' }
    $optional = [System.BitConverter]::ToUInt16($Bytes, [int] $offset + 20)
    if ($optional -lt 2 -or $offset + 24 + $optional -gt $Bytes.Length) { throw 'Command has an incomplete optional header.' }
    $flags = [System.BitConverter]::ToUInt16($Bytes, [int] $offset + 22)
    if ([System.BitConverter]::ToUInt32($Bytes, [int] $offset) -ne 0x4550 -or
        [System.BitConverter]::ToUInt16($Bytes, [int] $offset + 4) -ne 0x8664 -or
        [System.BitConverter]::ToUInt16($Bytes, [int] $offset + 24) -ne 0x20b -or
        -not ($flags -band 2) -or ($flags -band 0x2000)) { throw 'Command must be a PE32+ x86-64 executable.' }
}

function Assert-ZipContainer([byte[]] $Bytes) {
    # ZipArchive does not expose encryption flags. Check both inventories before
    # opening the entry. ZIP64 is unnecessary under this installer byte ceiling.
    $end = -1
    for ($i = $Bytes.Length - 22; $i -ge [Math]::Max(0, $Bytes.Length - 65557); $i--) {
        if ([System.BitConverter]::ToUInt32($Bytes, $i) -eq 0x06054b50 -and
            $i + 22 + [System.BitConverter]::ToUInt16($Bytes, $i + 20) -eq $Bytes.Length) { $end = $i; break }
    }
    if ($end -lt 0) { throw 'Archive has no complete central inventory.' }
    if ([System.BitConverter]::ToUInt16($Bytes, $end + 4) -ne 0 -or
        [System.BitConverter]::ToUInt16($Bytes, $end + 6) -ne 0 -or
        [System.BitConverter]::ToUInt16($Bytes, $end + 8) -ne 1 -or
        [System.BitConverter]::ToUInt16($Bytes, $end + 10) -ne 1) { throw 'Archive must have one entry on one disk.' }
    $offset = [long] [System.BitConverter]::ToUInt32($Bytes, $end + 16)
    $size = [long] [System.BitConverter]::ToUInt32($Bytes, $end + 12)
    if ($offset + $size -ne $end -or $size -lt 46 -or $offset + 46 -gt $end -or
        [System.BitConverter]::ToUInt32($Bytes, [int] $offset) -ne 0x02014b50) { throw 'Archive central inventory is malformed.' }
    $flags = [System.BitConverter]::ToUInt16($Bytes, [int] $offset + 8)
    $method = [System.BitConverter]::ToUInt16($Bytes, [int] $offset + 10)
    $local = [long] [System.BitConverter]::ToUInt32($Bytes, [int] $offset + 42)
    if (($flags -band 0x41) -or $method -notin @(0, 8) -or $local + 30 -gt $offset -or
        [System.BitConverter]::ToUInt32($Bytes, [int] $local) -ne 0x04034b50 -or
        [System.BitConverter]::ToUInt16($Bytes, [int] $local + 6) -ne $flags -or
        [System.BitConverter]::ToUInt16($Bytes, [int] $local + 8) -ne $method) { throw 'Archive member is encrypted, unsupported or inconsistent.' }
}

function Read-CommandZip([byte[]] $Bytes) {
    Assert-ZipContainer $Bytes
    Add-Type -AssemblyName System.IO.Compression
    $memory = [System.IO.MemoryStream]::new($Bytes, $false)
    $archive = $null
    try {
        $archive = [System.IO.Compression.ZipArchive]::new($memory, [System.IO.Compression.ZipArchiveMode]::Read)
        if ($archive.Entries.Count -ne 1 -or $archive.Entries[0].FullName -cne 'thinkthen.exe') { throw 'Archive must contain exactly thinkthen.exe.' }
        $entry = $archive.Entries[0]
        $attributes = [long] $entry.ExternalAttributes -band 0xffffffffL
        $kind = ($attributes -shr 16) -band 0xf000
        if ($kind -notin @(0, 0x8000) -or ($attributes -band 0x410)) { throw 'Archive command is not a regular file.' }
        if ($entry.Length -gt 134217728 -or $entry.CompressedLength -gt 134217728) { throw 'Archive member exceeds its byte limit.' }
        $stream = $entry.Open(); $output = [System.IO.MemoryStream]::new()
        try {
            $buffer = [byte[]]::new(65536)
            while (($count = $stream.Read($buffer, 0, $buffer.Length)) -gt 0) {
                if ($output.Length + $count -gt 134217728 -or $output.Length + $count -gt $entry.Length) { throw 'Expanded command exceeds its byte limit.' }
                $output.Write($buffer, 0, $count)
            }
            if ($output.Length -ne $entry.Length) { throw 'Expanded command differs from its declared length.' }
            $binary = $output.ToArray()
            Assert-PE $binary
            return ,$binary
        } finally { $stream.Dispose(); $output.Dispose() }
    } finally { if ($archive) { $archive.Dispose() }; $memory.Dispose() }
}

function Get-Attributes([string] $Path) {
    # GetAttributes observes a reparse point instead of resolving its target.
    try { return [System.IO.File]::GetAttributes($Path) }
    catch [System.IO.FileNotFoundException] { return $null }
    catch [System.IO.DirectoryNotFoundException] { return $null }
}

function Assert-Private([string] $Path, [bool] $Directory) {
    $attrs = Get-Attributes $Path
    if ($null -eq $attrs -or ($attrs -band [System.IO.FileAttributes]::ReparsePoint) -or
        [bool] ($attrs -band [System.IO.FileAttributes]::Directory) -ne $Directory) { throw "Refusing a missing, linked or nonregular path: $Path" }
    $acl = Get-Acl -LiteralPath $Path
    $rawAcl = [System.Security.AccessControl.RawSecurityDescriptor]::new($acl.GetSecurityDescriptorBinaryForm(), 0)
    if ($null -eq $rawAcl.DiscretionaryAcl) { throw "Path has an unrestricted access list. Inspect it or choose a new install directory: $Path" }
    if ($acl.GetOwner([System.Security.Principal.SecurityIdentifier]).Value -ne $script:UserSid) { throw "Path has a foreign owner. Inspect it or choose a new install directory: $Path" }
    foreach ($rule in $acl.GetAccessRules($true, $true, [System.Security.Principal.SecurityIdentifier])) {
        if ($rule.AccessControlType -eq [System.Security.AccessControl.AccessControlType]::Allow -and $rule.IdentityReference.Value -notin @($script:UserSid, 'S-1-5-18')) { throw "Path allows another identity. Inspect it or choose a new install directory: $Path" }
    }
}

function Assert-Ancestors([string] $Path) {
    $current = $Path
    while ($current) {
        $attrs = Get-Attributes $current
        if ($null -ne $attrs) {
            if (($attrs -band [System.IO.FileAttributes]::ReparsePoint) -or -not ($attrs -band [System.IO.FileAttributes]::Directory)) { throw "Refusing a linked or non-directory ancestor: $current" }
            $acl = Get-Acl -LiteralPath $current
            $rawAcl = [System.Security.AccessControl.RawSecurityDescriptor]::new($acl.GetSecurityDescriptorBinaryForm(), 0)
            if ($null -eq $rawAcl.DiscretionaryAcl) { throw "Refusing an ancestor with unrestricted access: $current" }
            $owner = $acl.GetOwner([System.Security.Principal.SecurityIdentifier]).Value
            if ($owner -notin @($script:UserSid, 'S-1-5-18', 'S-1-5-32-544', 'S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464')) { throw "Refusing a foreign-owned ancestor: $current" }
            # WriteData alone permits a child name, but does not replace an
            # existing private child. DeleteChild, Delete, ACL and owner changes do.
            $danger = [System.Security.AccessControl.FileSystemRights] 'DeleteSubdirectoriesAndFiles, Delete, ChangePermissions, TakeOwnership'
            foreach ($rule in $acl.GetAccessRules($true, $true, [System.Security.Principal.SecurityIdentifier])) {
                if (($rule.PropagationFlags -band [System.Security.AccessControl.PropagationFlags]::InheritOnly)) { continue }
                if ($rule.AccessControlType -eq [System.Security.AccessControl.AccessControlType]::Allow -and
                    $rule.IdentityReference.Value -notin @($script:UserSid, 'S-1-5-18', 'S-1-5-32-544', 'S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464') -and
                    ($rule.FileSystemRights -band $danger)) { throw "Another user can replace an install ancestor: $current" }
            }
        }
        $parent = [System.IO.Directory]::GetParent($current)
        if ($null -eq $parent) { break }
        $current = $parent.FullName
    }
}

function Get-InstallDirectory([string] $Path) {
    if (-not $Path -or $Path -notmatch '^[a-zA-Z]:\\' -or $Path.Substring(2).Contains(':') -or $Path.Contains('/') -or $Path -match '[\x00-\x1f]') { throw 'Install directory must be an absolute local NTFS path, without streams or device names.' }
    foreach ($part in $Path.Substring(3).Split('\')) {
        if ($part -in @('.', '..') -or $part -match '[ .]$' -or $part -match '^(?i:CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])(?:\.|$)') { throw 'Refusing an ambiguous Windows install path.' }
    }
    $full = [System.IO.Path]::GetFullPath($Path).TrimEnd('\')
    if ($full.Length -lt 4) { throw 'Refusing a drive root install directory.' }
    $drive = [System.IO.DriveInfo]::new([System.IO.Path]::GetPathRoot($full))
    if ($drive.DriveType -eq [System.IO.DriveType]::Network -or $drive.DriveFormat -ne 'NTFS') { throw 'Install directory must use a local NTFS volume.' }
    Assert-Ancestors $full
    return $full
}

function New-PrivateDirectory([string] $Path) {
    Assert-Ancestors $Path
    [ThinkThenInstall.Native]::PrivateDirectory($Path, $script:UserSid)
    Assert-Private $Path $true
}

function Ensure-InstallDirectory([string] $Path) {
    if ($null -ne (Get-Attributes $Path)) { Assert-Private $Path $true; return }
    $parent = [System.IO.Directory]::GetParent($Path).FullName
    if ($null -eq (Get-Attributes $parent)) { Ensure-InstallDirectory $parent }
    New-PrivateDirectory $Path
}

function Get-FileHashPrivate([string] $Path) {
    Assert-Ancestors ([System.IO.Path]::GetDirectoryName($Path))
    Assert-Private $Path $false
    $stream = [System.IO.File]::Open($Path, [System.IO.FileMode]::Open, [System.IO.FileAccess]::Read, [System.IO.FileShare]::Read)
    $hash = [System.Security.Cryptography.SHA256]::Create()
    try { return ([System.BitConverter]::ToString($hash.ComputeHash($stream))).Replace('-', '').ToLowerInvariant() }
    finally { $hash.Dispose(); $stream.Dispose() }
}

function Write-PrivateFile([string] $Path, [byte[]] $Bytes) {
    Assert-Ancestors ([System.IO.Path]::GetDirectoryName($Path))
    Assert-Private ([System.IO.Path]::GetDirectoryName($Path)) $true
    $stream = [ThinkThenInstall.Native]::NewFile($Path, $script:UserSid, $false)
    $script:Owned.Add($Path) | Out-Null
    try { $stream.Write($Bytes, 0, $Bytes.Length); $stream.Flush($true) } finally { $stream.Dispose() }
    Assert-Private $Path $false
}

function Read-Receipt([string] $Path, [string] $Executable) {
    Assert-Ancestors ([System.IO.Path]::GetDirectoryName($Path))
    Assert-Private $Path $false
    $stream = [System.IO.File]::Open($Path, [System.IO.FileMode]::Open, [System.IO.FileAccess]::Read, [System.IO.FileShare]::Read)
    try {
        if ($stream.Length -gt 16384) { throw 'Existing receipt exceeds its byte limit.' }
        $bytes = [byte[]]::new([int] $stream.Length)
        $count = 0
        while ($count -lt $bytes.Length) {
            $read = $stream.Read($bytes, $count, $bytes.Length - $count)
            if ($read -eq 0) { throw 'Existing receipt ended early.' }
            $count += $read
        }
    } finally { $stream.Dispose() }
    try { $receipt = [System.Text.Encoding]::UTF8.GetString($bytes) | ConvertFrom-Json } catch { throw 'Existing receipt is malformed.' }
    $names = @($receipt.PSObject.Properties.Name | Sort-Object)
    if (($names -join ',') -ne 'executable_path,new_sha256,old_sha256,schema_version,state,version' -or
        ($receipt.schema_version -isnot [int] -and $receipt.schema_version -isnot [long]) -or $receipt.schema_version -ne 1 -or $receipt.state -cnotin @('pending', 'installed') -or
        $receipt.executable_path -isnot [string] -or $receipt.executable_path -cne $Executable -or
        $receipt.new_sha256 -isnot [string] -or $receipt.new_sha256 -cnotmatch '\A[a-f0-9]{64}\z' -or
        $receipt.old_sha256 -isnot [string] -or $receipt.old_sha256 -cnotmatch '\A(?:[a-f0-9]{64})?\z' -or
        $receipt.version -isnot [string]) { throw 'Existing receipt has an unknown schema, state, path or digest.' }
    $null = Convert-ReleaseVersion $receipt.version
    return $receipt
}

function Get-CurrentDigest([string] $Path) {
    if ($null -eq (Get-Attributes $Path)) { return '' }
    return Get-FileHashPrivate $Path
}

function Assert-ReceiptState([string] $ReceiptPath, [string] $Executable, [string] $Current, [string] $Directory) {
    if ($null -eq (Get-Attributes $ReceiptPath)) {
        # Observe prior scratch names only. Never open or follow earlier entries.
        foreach ($name in [System.IO.Directory]::EnumerateFileSystemEntries($Directory, '.thinkthen-install.*')) {
            if ([System.IO.Path]::GetFileName($name) -match '^\.thinkthen-install\.[a-f0-9-]{36}$') { throw 'Receipt is absent beside earlier installer scratch. Inspect recovery artifacts manually before retrying.' }
        }
        return
    }
    $receipt = Read-Receipt $ReceiptPath $Executable
    if ($receipt.state -ceq 'installed') {
        if (-not $Current -or $receipt.new_sha256 -cne $Current) { throw 'Installed receipt does not match the command. Inspect recovery artifacts manually.' }
    } else {
        if (-not $Current -and $receipt.old_sha256) { throw 'Pending upgrade command is absent. Recover manually from inspected snapshots.' }
        if ($Current -cne $receipt.old_sha256 -and $Current -cne $receipt.new_sha256) { throw 'Pending receipt does not match the command. Recover manually.' }
    }
}

function Save-Snapshot([string] $Path, [string] $Digest, [string] $Name) {
    if (-not $Digest) { return '' }
    $snapshot = Join-Path $script:Scratch $Name
    Assert-Private $Path $false
    # Bound the original command as well as downloaded executable bytes.
    $info = [System.IO.FileInfo]::new($Path)
    if ($info.Length -gt 134217728) { throw 'Existing file exceeds snapshot limit.' }
    Write-PrivateFile $snapshot ([System.IO.File]::ReadAllBytes($Path))
    if ((Get-FileHashPrivate $snapshot) -cne $Digest -or (Get-FileHashPrivate $Path) -cne $Digest) { throw 'Original file changed while taking its snapshot.' }
    return $snapshot
}

function Invoke-FileReplace([string] $Stage, [string] $Destination, [string] $Backup) {
    [System.IO.File]::Replace($Stage, $Destination, $Backup, $false)
}

function Get-ReplacementState([string] $Destination, [string] $Stage, [string] $Backup) {
    $state = @{}
    foreach ($path in @($Destination, $Stage, $Backup)) {
        if (-not $path) { continue }
        try { $state[$path] = Get-CurrentDigest $path }
        catch { $state[$path] = 'uninspectable'; [Console]::Error.WriteLine("Recovery inspection failed for {0}: {1}", $path, $_.Exception.Message) }
    }
    return $state
}

function Commit-File([string] $Stage, [string] $Destination, [string] $Expected) {
    Assert-Ancestors $script:InstallDirectory
    Assert-Private $script:InstallDirectory $true
    Assert-Private $Stage $false
    if ((Get-CurrentDigest $Destination) -cne $Expected) { throw 'Destination changed before replacement.' }
    $backup = Join-Path $script:Scratch ('backup-' + [guid]::NewGuid().ToString('D'))
    if ($null -ne (Get-Attributes $backup)) { throw 'Replacement backup path already exists.' }
    # The exact backup name belongs to this invocation, even if Replace throws.
    $script:Owned.Add($backup) | Out-Null
    try {
        if ($Expected) { Invoke-FileReplace $Stage $Destination $backup }
        else { [System.IO.File]::Move($Stage, $Destination) }
        $null = Get-FileHashPrivate $Destination
        if ($null -ne (Get-Attributes $backup)) { Assert-Private $backup $false }
    } catch {
        $first = $_
        $script:LastReplacementState = Get-ReplacementState $Destination $Stage $backup
        throw $first
    }
}

function Restore-File([string] $Destination, [string] $Snapshot, [string] $Original) {
    $current = Get-CurrentDigest $Destination
    if ($current -ceq $Original) { return }
    if (-not $Original) {
        Assert-Ancestors $script:InstallDirectory
        Assert-Private $Destination $false
        # Move new content into owned scratch instead of deleting the destination.
        $removed = Join-Path $script:Scratch ('removed-' + [guid]::NewGuid().ToString('D'))
        $script:Owned.Add($removed) | Out-Null
        try { [System.IO.File]::Move($Destination, $removed) }
        catch { $first = $_; $script:LastReplacementState = Get-ReplacementState $Destination $removed ''; throw $first }
    } else {
        if ((Get-FileHashPrivate $Snapshot) -cne $Original) { throw 'Recovery snapshot digest changed.' }
        $stage = Join-Path $script:Scratch ('restore-' + [guid]::NewGuid().ToString('D'))
        Write-PrivateFile $stage ([System.IO.File]::ReadAllBytes($Snapshot))
        Commit-File $stage $Destination $current
    }
    if ((Get-CurrentDigest $Destination) -cne $Original) { throw 'Recovery did not restore original file bytes.' }
}

function Remove-OwnedScratch([string] $Path) {
    if (-not $script:Owned.Contains($Path) -or $Path -cne $script:Scratch -or
        [System.IO.Path]::GetDirectoryName($Path) -cne $script:InstallDirectory -or
        [System.IO.Path]::GetFileName($Path) -cnotmatch '^\.thinkthen-install\.[a-f0-9-]{36}$') { throw 'Cleanup refuses a path this run did not create.' }
    Assert-Ancestors $script:InstallDirectory
    Assert-Private $Path $true
    # Inspect the whole flat inventory before deleting any file. Reparse points,
    # directories and unowned injected files stop cleanup without recursion.
    $files = @([System.IO.Directory]::EnumerateFileSystemEntries($Path))
    foreach ($file in $files) {
        if (-not $script:Owned.Contains($file) -or [System.IO.Path]::GetDirectoryName($file) -cne $Path) { throw 'Cleanup refuses an unowned scratch entry.' }
        Assert-Private $file $false
    }
    foreach ($file in $files) { Assert-Private $file $false; [System.IO.File]::Delete($file) }
    [System.IO.Directory]::Delete($Path, $false)
}

function Invoke-ThinkThenInstall([string] $RequestedVersion) {
    $script:Committed = $false
    if ([Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT -or -not [Environment]::Is64BitOperatingSystem -or
        $env:PROCESSOR_ARCHITECTURE -notin @('AMD64', 'x86') -or $env:PROCESSOR_ARCHITEW6432 -eq 'ARM64' -or
        ($PSVersionTable.PSVersion.Major -ne 5 -and $PSVersionTable.PSVersion.Major -ne 7) -or
        ($PSVersionTable.PSVersion.Major -eq 5 -and $PSVersionTable.PSVersion.Minor -lt 1)) { throw 'Installer requires Windows x86-64 with Windows PowerShell 5.1 or PowerShell 7.' }
    if ($RequestedVersion) { $RequestedVersion = Convert-ReleaseVersion $RequestedVersion }
    $base = Get-Base $env:THINKTHEN_INSTALL_BASE 'https://github.com'
    $api = Get-Base $env:THINKTHEN_INSTALL_API 'https://api.github.com'
    Initialize-InstallHelpers
    if (-not [ThinkThenInstall.Native]::X64()) { throw 'Installer requires native Windows x86-64.' }
    $script:UserSid = [System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value
    $directory = $env:THINKTHEN_INSTALL_DIR
    if (-not $directory) { $directory = Join-Path $env:LOCALAPPDATA 'Programs\thinkthen' }
    $script:InstallDirectory = Get-InstallDirectory $directory
    Ensure-InstallDirectory $script:InstallDirectory
    $installed = Join-Path $script:InstallDirectory 'thinkthen.exe'
    $receiptPath = Join-Path $script:InstallDirectory 'thinkthen.install.json'
    $lockPath = Join-Path $script:InstallDirectory '.thinkthen-install.lock'
    $script:Owned = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)
    $script:Scratch = ''; $lock = $null; $retain = $false
    try {
        Assert-Ancestors $script:InstallDirectory
        Assert-Private $script:InstallDirectory $true
        if ($null -ne (Get-Attributes $lockPath)) { Assert-Private $lockPath $false }
        # Supply file owner and access rules at creation even under an elevated
        # token whose default file owner would be the Administrators group.
        if ($null -eq (Get-Attributes $lockPath)) { $lock = [ThinkThenInstall.Native]::NewFile($lockPath, $script:UserSid, $true) }
        else { $lock = [System.IO.File]::Open($lockPath, [System.IO.FileMode]::Open, [System.IO.FileAccess]::ReadWrite, [System.IO.FileShare]::None) }
        Assert-Private $lockPath $false
        # Inspect both FILE owners and access lists before any read or hash.
        foreach ($path in @($installed, $receiptPath)) {
            if ($null -ne (Get-Attributes $path)) { Assert-Private $path $false }
        }
        $oldHash = Get-CurrentDigest $installed
        Assert-ReceiptState $receiptPath $installed $oldHash $script:InstallDirectory
        $oldReceiptHash = Get-CurrentDigest $receiptPath
        if (-not $RequestedVersion) { $RequestedVersion = Get-LatestVersion $api }
        $asset = "thinkthen-$RequestedVersion-x86_64-pc-windows-msvc.zip"
        $url = "$base/botassembly/thinkthen/releases/download/v$RequestedVersion/$asset"
        $zip = Get-Download ([uri] $url) 134217728
        $checksum = Get-Download ([uri] "$url.sha256") 4096
        Assert-Checksum $zip $checksum $asset
        $binary = Read-CommandZip $zip
        $newHash = Get-BytesHash $binary
        $script:Scratch = Join-Path $script:InstallDirectory ('.thinkthen-install.' + [guid]::NewGuid().ToString('D'))
        New-PrivateDirectory $script:Scratch
        $script:Owned.Add($script:Scratch) | Out-Null
        $stage = Join-Path $script:Scratch 'command.exe'
        Write-PrivateFile $stage $binary
        if ((Get-FileHashPrivate $stage) -cne $newHash) { throw 'Staged command checksum changed.' }
        $reported = [ThinkThenInstall.Native]::Probe($stage)
        if ((Get-FileHashPrivate $stage) -cne $newHash) { throw 'Staged command changed during its version probe.' }
        if ($reported -cnotmatch ('\Athinkthen ' + [regex]::Escape($RequestedVersion) + '(?:\r?\n)?\z')) { throw 'Staged command reports another version.' }
        $oldSnapshot = Save-Snapshot $installed $oldHash 'original-command.exe'
        $receiptSnapshot = Save-Snapshot $receiptPath $oldReceiptHash 'original-receipt.json'
        $pendingStage = Join-Path $script:Scratch 'pending.json'
        $installedStage = Join-Path $script:Scratch 'installed.json'
        foreach ($state in @('pending', 'installed')) {
            $receipt = [ordered] @{ schema_version = 1; state = $state; executable_path = $installed; version = $RequestedVersion; old_sha256 = $oldHash; new_sha256 = $newHash }
            $json = $receipt | ConvertTo-Json -Compress
            Write-PrivateFile (Join-Path $script:Scratch "$state.json") ([System.Text.Encoding]::UTF8.GetBytes($json))
        }
        # Recheck both original identities just before the first commit attempt.
        if ((Get-CurrentDigest $installed) -cne $oldHash -or (Get-CurrentDigest $receiptPath) -cne $oldReceiptHash) { throw 'Installation changed during preparation.' }
        $pendingHash = Get-FileHashPrivate $pendingStage
        try {
            Commit-File $pendingStage $receiptPath $oldReceiptHash
            Commit-File $stage $installed $oldHash
            if ((Get-FileHashPrivate $installed) -cne $newHash) { throw 'Installed command digest changed.' }
            Commit-File $installedStage $receiptPath $pendingHash
            $script:Committed = $true
        } catch {
            $primary = $_
            try {
                Restore-File $installed $oldSnapshot $oldHash
                Restore-File $receiptPath $receiptSnapshot $oldReceiptHash
            } catch {
                $retain = $true
                try { [Console]::Error.WriteLine('Recovery could not restore the original installation: ' + $_.Exception.Message) } catch {}
                # Restore pending evidence when possible. Its old/new digests
                # describe the failed transaction, never a guessed installed state.
                try {
                    $pendingRecovery = Join-Path $script:Scratch ('recovery-pending-' + [guid]::NewGuid().ToString('D') + '.json')
                    $pending = [ordered] @{ schema_version = 1; state = 'pending'; executable_path = $installed; version = $RequestedVersion; old_sha256 = $oldHash; new_sha256 = $newHash }
                    Write-PrivateFile $pendingRecovery ([System.Text.Encoding]::UTF8.GetBytes(($pending | ConvertTo-Json -Compress)))
                    Commit-File $pendingRecovery $receiptPath (Get-CurrentDigest $receiptPath)
                } catch { try { [Console]::Error.WriteLine('Pending receipt recovery failed: ' + $_.Exception.Message) } catch {} }
                # Receipt inspection and diagnostics must not replace the first failure.
                try {
                    if ($null -eq (Get-Attributes $receiptPath)) {
                        try { [Console]::Error.WriteLine('Receipt is absent. Manual recovery is required.') } catch {}
                    }
                } catch {
                    try { [Console]::Error.WriteLine('Receipt inspection failed. Manual recovery is required: ' + $_.Exception.Message) } catch {}
                }
                try { [Console]::Error.WriteLine('Retained recovery artifacts: ' + $script:Scratch) } catch {}
            }
            throw $primary
        }
        [Console]::WriteLine("Installed thinkthen {0} to {1}", $RequestedVersion, $installed)
        if (@($env:PATH -split ';') -notcontains $script:InstallDirectory) {
            $quoted = $script:InstallDirectory.Replace("'", "''")
            [Console]::WriteLine("For this PowerShell session: `$env:PATH = '{0};' + `$env:PATH", $quoted)
            [Console]::WriteLine('For future sessions, add the install directory through Windows environment settings.')
        }
        [Console]::WriteLine('To remove ThinkThen, delete thinkthen.exe, thinkthen.install.json and .thinkthen-install.lock from this directory.')
    } finally {
        if ($script:Scratch -and -not $retain) {
            try { Remove-OwnedScratch $script:Scratch }
            catch { try { [Console]::Error.WriteLine('Cleanup left owned scratch for inspection: ' + $_.Exception.Message) } catch {} }
        }
        if ($lock) { $lock.Dispose() }
    }
}

# Entry point. Fixtures load the actual function ASTs without executing this call.
try { Invoke-ThinkThenInstall $Version }
catch {
    [Console]::Error.WriteLine('thinkthen install: ' + $_.Exception.Message)
    if ($script:Committed) { exit 0 }
    exit 1
}
