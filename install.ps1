# Install the command from a release after verifying its checksum.
# irm https://thinkthen.dev/install.ps1 | iex
# & .\install.ps1 --version 0.2.0
# Adds the install folder to HKCU Environment\Path and this session unless
# THINKTHEN_NO_MODIFY_PATH=1. The receipt records the registry value to remove
# the folder from when uninstalling; the machine PATH is never changed.
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

function Assert-InstallPath([string] $Path) {
    if ($Path -notmatch '^[A-Za-z]:\\' -or $Path -match '["\x00-\x1f]' -or $Path.Substring(2).Contains(':')) {
        throw 'Install directory must be an absolute local path without quotes, controls or streams.'
    }
    foreach ($part in $Path.Substring(3).Split('\')) {
        if ($part -in @('.', '..') -or $part -match '[ .]$' -or $part -match '^(?i:CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])(?:\.|$)') {
            throw 'Refusing an ambiguous install directory.'
        }
    }
    $full = [IO.Path]::GetFullPath($Path).TrimEnd('\')
    if ($full.Length -lt 4) { throw 'Refusing a drive root install directory.' }
    $drive = [IO.DriveInfo]::new([IO.Path]::GetPathRoot($full))
    if ($drive.DriveType -eq 'Network' -or $drive.DriveFormat -ne 'NTFS') { throw 'Install directory must use local NTFS.' }
    $current = $full
    while ($current) {
        if (Test-Path -LiteralPath $current) {
            $item = Get-Item -LiteralPath $current -Force
            if (-not $item.PSIsContainer -or ($item.Attributes -band [IO.FileAttributes]::ReparsePoint)) {
                throw "Refusing a symbolic-link or non-directory install folder: $current"
            }
        }
        $current = [IO.Path]::GetDirectoryName($current)
    }
    return $full
}

function Assert-RegularFile([string] $Path) {
    if (Test-Path -LiteralPath $Path) {
        $item = Get-Item -LiteralPath $Path -Force
        if ($item.PSIsContainer -or ($item.Attributes -band [IO.FileAttributes]::ReparsePoint)) {
            throw "Refusing a symbolic-link binary or receipt: $Path"
        }
    }
}

function Test-PathEntry([string] $Value, [string] $Directory) {
    foreach ($entry in ($Value -split ';')) {
        $expanded = [Environment]::ExpandEnvironmentVariables($entry).Trim('"').TrimEnd('\')
        if (-not [IO.Path]::IsPathRooted($expanded)) { continue }
        try { $expanded = [IO.Path]::GetFullPath($expanded).TrimEnd('\') } catch { continue }
        if ($expanded -ieq $Directory.TrimEnd('\')) { return $true }
    }
    return $false
}

function Add-UserPath([string] $Directory) {
    $changed = $false
    $key = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey('Environment')
    try {
        $kind = [Microsoft.Win32.RegistryValueKind]::ExpandString
        $raw = ''
        if ($key.GetValueNames() -contains 'Path') {
            $kind = $key.GetValueKind('Path')
            if ($kind -notin @([Microsoft.Win32.RegistryValueKind]::String, [Microsoft.Win32.RegistryValueKind]::ExpandString)) {
                throw 'User PATH must be a string or expandable string; its value was not changed.'
            }
            $raw = [string] $key.GetValue('Path', '', [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
        }
        if (-not (Test-PathEntry $raw $Directory)) {
            $updated = $Directory
            if ($raw.Length) { $updated = $raw + ';' + $Directory }
            $key.SetValue('Path', $updated, $kind)
            $changed = $true
            Write-Host "Added $Directory to user PATH (HKCU\Environment\Path). New windows pick it up."
        }
    } finally { $key.Dispose() }
    if (-not (Test-PathEntry $env:Path $Directory)) { $env:Path = $Directory + ';' + $env:Path }
    if ($changed) {
        if (-not ('ThinkThenInstaller.EnvironmentChange' -as [type])) {
            Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
namespace ThinkThenInstaller {
    public static class EnvironmentChange {
        [DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
        public static extern IntPtr SendMessageTimeout(IntPtr window, uint message,
            IntPtr parameter, string value, uint flags, uint timeout, out IntPtr result);
    }
}
'@
        }
        $result = [IntPtr]::Zero
        $null = [ThinkThenInstaller.EnvironmentChange]::SendMessageTimeout(
            [IntPtr] 0xffff, 0x001a, [IntPtr]::Zero, 'Environment', 2, 5000, [ref] $result)
    }
    return $changed
}

$requested = 'latest'
for ($index = 0; $index -lt $args.Count; $index++) {
    switch -CaseSensitive ($args[$index]) {
        { $_ -in @('--version', '-V', '-Version') } {
            $index++
            if ($index -ge $args.Count) { throw '--version needs a value' }
            $requested = ([string] $args[$index]) -creplace '^v', ''
        }
        { $_ -in @('--help', '-h') } { Write-Output 'usage: install.ps1 [--version X.Y.Z]'; return }
        default { throw "Unknown argument: $($args[$index])" }
    }
}

$work = $null
$stage = $null
$receiptStage = $null
try {
    if ([Environment]::OSVersion.Platform -ne 'Win32NT' -or
        ($env:PROCESSOR_ARCHITECTURE -ne 'AMD64' -and $env:PROCESSOR_ARCHITEW6432 -ne 'AMD64')) {
        throw 'Installer requires Windows x86-64.'
    }
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    $base = 'https://github.com'
    if ($env:THINKTHEN_INSTALL_BASE) { $base = $env:THINKTHEN_INSTALL_BASE.TrimEnd('/') }
    $api = 'https://api.github.com'
    if ($env:THINKTHEN_INSTALL_API) { $api = $env:THINKTHEN_INSTALL_API.TrimEnd('/') }
    $directory = Join-Path $HOME '.local\bin'
    if ($env:THINKTHEN_INSTALL_DIR) { $directory = $env:THINKTHEN_INSTALL_DIR }
    $directory = Assert-InstallPath $directory
    if ($env:THINKTHEN_NO_MODIFY_PATH -ne '1' -and $directory.Contains(';')) {
        throw 'Install directory contains a PATH separator; set THINKTHEN_NO_MODIFY_PATH=1 for an explicit-path install.'
    }
    $installed = Join-Path $directory 'thinkthen.exe'
    $receipt = Join-Path $directory 'thinkthen.install.json'
    Assert-RegularFile $installed
    Assert-RegularFile $receipt

    if ($requested -eq 'latest') {
        try {
            $releases = Invoke-RestMethod "$api/repos/botassembly/thinkthen/releases"
            foreach ($release in $releases) {
                if ($release.draft -or $release.prerelease) { continue }
                $candidate = $release.tag_name -creplace '^v', ''
                if ($candidate -notmatch '^[0-9]+\.[0-9]+\.[0-9]+$' -or [version] $candidate -lt [version] '0.2.0') { continue }
                if (@($release.assets | ForEach-Object { $_.name }) -ccontains "thinkthen-$candidate-windows-x86_64.zip") {
                    $requested = $candidate
                    break
                }
            }
        } catch { }
        if ($requested -eq 'latest') {
            $response = Invoke-WebRequest -UseBasicParsing -Method Head "$base/botassembly/thinkthen/releases/latest"
            if ($response.BaseResponse.PSObject.Properties['ResponseUri']) { $url = $response.BaseResponse.ResponseUri.AbsoluteUri }
            else { $url = $response.BaseResponse.RequestMessage.RequestUri.AbsoluteUri }
            if ($url -notmatch '/releases/tag/v?([0-9]+\.[0-9]+\.[0-9]+)$') { throw 'Latest release redirect named no version.' }
            $requested = $Matches[1]
        }
    }
    if ($requested -notmatch '^[0-9]+\.[0-9]+\.[0-9]+$') { throw 'Give a release version as X.Y.Z.' }
    if ([version] $requested -lt [version] '0.2.0') { throw 'Windows archives start with 0.2. Give a version of at least 0.2.0.' }
    $asset = "thinkthen-$requested-windows-x86_64.zip"
    $releaseUrl = "$base/botassembly/thinkthen/releases/download/v$requested"
    $work = Join-Path ([IO.Path]::GetTempPath()) ('thinkthen-install-' + [guid]::NewGuid())
    $null = New-Item -ItemType Directory -Path $work
    $archive = Join-Path $work $asset
    Invoke-WebRequest -UseBasicParsing "$releaseUrl/$asset" -OutFile $archive
    $checksumUrl = "$releaseUrl/SHA256SUMS"
    $checksumPath = Join-Path $work 'checksums'
    Invoke-WebRequest -UseBasicParsing $checksumUrl -OutFile $checksumPath
    $records = @(Get-Content -LiteralPath $checksumPath | Where-Object { $_.Trim() })
    $pattern = '^([a-fA-F0-9]{64})[ \t]+\*?' + [regex]::Escape($asset) + '[ \t]*$'
    $expected = @($records | ForEach-Object { if ($_ -cmatch $pattern) { $Matches[1].ToLowerInvariant() } })
    if ($expected.Count -ne 1) { throw 'No unique valid checksum for this archive; refusing an unverified install.' }
    if ((Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant() -cne $expected[0]) {
        throw 'Archive checksum does not match; refusing to install.'
    }
    # Extract only the command so archive paths cannot escape the temporary folder.
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $zip = [IO.Compression.ZipFile]::OpenRead($archive)
    try {
        $entries = @($zip.Entries | Where-Object { $_.FullName -ceq 'thinkthen.exe' })
        if ($entries.Count -ne 1) { throw 'Archive holds no unique thinkthen.exe.' }
        $binary = Join-Path $work 'thinkthen.exe'
        [IO.Compression.ZipFileExtensions]::ExtractToFile($entries[0], $binary)
    } finally { $zip.Dispose() }
    $null = New-Item -ItemType Directory -Path $directory -Force
    $null = Assert-InstallPath $directory
    Assert-RegularFile $installed
    Assert-RegularFile $receipt
    $previous = $null
    if (Test-Path -LiteralPath $receipt) {
        $previous = Get-Content -LiteralPath $receipt -Raw | ConvertFrom-Json
        if ($previous.state -eq 'pending') {
            $currentHash = ''
            if (Test-Path -LiteralPath $installed) { $currentHash = (Get-FileHash -LiteralPath $installed -Algorithm SHA256).Hash.ToLowerInvariant() }
            if ($previous.executable_path -cne $installed -or $currentHash -cnotin @('', $previous.old_sha256, $previous.new_sha256)) {
                throw 'Pending receipt does not match the installed binary.'
            }
        }
    }
    $stage = Join-Path $directory ('.thinkthen-stage-' + [guid]::NewGuid() + '.exe')
    Copy-Item -LiteralPath $binary -Destination $stage
    $reported = & $stage --version
    if ($LASTEXITCODE -ne 0 -or $reported -isnot [string] -or $reported -cnotmatch ('^thinkthen ' + [regex]::Escape($requested) + '(?: \([^\r\n]*\))?$')) { throw 'Staged command reports another version or did not run.' }
    $oldHash = ''
    if (Test-Path -LiteralPath $installed) { $oldHash = (Get-FileHash -LiteralPath $installed -Algorithm SHA256).Hash.ToLowerInvariant() }
    $record = [ordered] @{ schema_version = 1; state = 'pending'; executable_path = $installed; version = $requested; old_sha256 = $oldHash; new_sha256 = (Get-FileHash -LiteralPath $stage -Algorithm SHA256).Hash.ToLowerInvariant() }
    $receiptStage = Join-Path $directory ('.thinkthen-receipt-' + [guid]::NewGuid())
    $record | ConvertTo-Json | Set-Content -LiteralPath $receiptStage -Encoding UTF8
    Move-Item -LiteralPath $receiptStage -Destination $receipt -Force
    Move-Item -LiteralPath $stage -Destination $installed -Force
    $pathChanges = @()
    if ($previous -and $previous.PSObject.Properties['path_changes'] -and
        @($previous.path_changes) -contains 'HKCU\Environment\Path') {
        $pathChanges = @('HKCU\Environment\Path')
    }
    if ($env:THINKTHEN_NO_MODIFY_PATH -ne '1') {
        if (Add-UserPath $directory) { $pathChanges = @('HKCU\Environment\Path') }
    }
    $record.path_changes = @($pathChanges)
    $record.state = 'installed'
    $record | ConvertTo-Json | Set-Content -LiteralPath $receiptStage -Encoding UTF8
    Move-Item -LiteralPath $receiptStage -Destination $receipt -Force
    Write-Output "Installed $reported to $installed"
    if (-not (Test-PathEntry $env:PATH $directory)) {
        $quoted = $directory.Replace("'", "''")
        Write-Output "For this session: `$env:PATH = '$quoted;' + `$env:PATH"
    }
} catch {
    throw ('thinkthen install: ' + $_.Exception.Message)
} finally {
    if ($stage -and (Test-Path -LiteralPath $stage)) { Remove-Item -LiteralPath $stage -Force }
    if ($receiptStage -and (Test-Path -LiteralPath $receiptStage)) { Remove-Item -LiteralPath $receiptStage -Force }
    if ($work -and (Test-Path -LiteralPath $work)) { Remove-Item -LiteralPath $work -Recurse -Force }
}
