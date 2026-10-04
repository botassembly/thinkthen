# Run by windows-installer-test.py. These are actual installer function ASTs.
param([string] $Installer, [string] $Cases, [string] $FixtureRoot, [string] $NativeBinary = '', [string] $ReleaseVersion = '0.2.0', [string] $ReleaseBase = '')
Set-StrictMode -Version 2.0
$ErrorActionPreference = 'Stop'
$tokens = $null; $errors = $null
$ast = [System.Management.Automation.Language.Parser]::ParseFile($Installer, [ref] $tokens, [ref] $errors)
if ($errors.Count) { throw ($errors | Out-String) }
foreach ($function in $ast.EndBlock.Statements) {
    if ($function -is [System.Management.Automation.Language.FunctionDefinitionAst]) { . ([scriptblock]::Create($function.Extent.Text)) }
}
Initialize-InstallHelpers
$script:Checks = 0
function Check([bool] $Pass, [string] $Label) {
    if (-not $Pass) { throw ('Failed: ' + $Label) }
    $script:Checks++
}
function Refuses([scriptblock] $Call, [string] $Label) {
    $refused = $false
    try { & $Call | Out-Null } catch { $refused = $true }
    Check $refused $Label
}
$table = Get-Content -LiteralPath $Cases -Raw | ConvertFrom-Json
foreach ($case in $table) {
    $refused = $false
    try {
        switch ($case.kind) {
            version { $null = Convert-ReleaseVersion $case.value }
            transport { Assert-Transport ([uri] $case.value) }
            base { $null = Get-Base $case.value 'https://github.com' }
            pe { Assert-PE ([System.IO.File]::ReadAllBytes((Join-Path $FixtureRoot $case.file))) }
            zip { $null = Read-CommandZip ([System.IO.File]::ReadAllBytes((Join-Path $FixtureRoot $case.file))) }
            checksum {
                $bytes = [System.IO.File]::ReadAllBytes((Join-Path $FixtureRoot $case.file))
                Assert-Checksum $bytes ([System.Text.Encoding]::UTF8.GetBytes($case.value)) $case.name
            }
            download { $null = Get-Download ([uri] $case.value) $case.limit }
            latest { Check ((Get-LatestVersion $case.value) -ceq $case.expected) $case.label }
            default { throw ('Unknown case: ' + $case.kind) }
        }
    } catch { $refused = $true; if ($case.ok) { throw } }
    Check ($refused -ne $case.ok) $case.label
}
# This plant precedes every real cleanup. No ACL stub is involved.
$script:InstallDirectory = $FixtureRoot
$script:Scratch = Join-Path $FixtureRoot '.thinkthen-install.00000000-0000-0000-0000-000000000000'
$script:Owned = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)
$script:Owned.Add($script:Scratch) | Out-Null
Refuses { Remove-OwnedScratch ([System.IO.Path]::GetDirectoryName($Installer)) } 'cleanup refuses actual lane'
Refuses { Remove-OwnedScratch (Join-Path $FixtureRoot 'unowned-sibling') } 'cleanup refuses sibling'
Check ([System.IO.Directory]::Exists([System.IO.Path]::GetDirectoryName($Installer))) 'lane survives cleanup plant'

if ($NativeBinary) {
    if ([Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT) { throw 'Native proof requires Windows.' }
    $script:UserSid = [System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value
    # Fixture root is owned by Python. Installer cleanup owns only its own flat scratch.
    $nativeRoot = Join-Path $FixtureRoot ('native-' + [guid]::NewGuid().ToString('D'))
    New-PrivateDirectory $nativeRoot
    $script:InstallDirectory = $nativeRoot
    $script:Scratch = Join-Path $nativeRoot ('.thinkthen-install.' + [guid]::NewGuid().ToString('D'))
    New-PrivateDirectory $script:Scratch
    $script:Owned.Add($script:Scratch) | Out-Null
    $original = Join-Path $nativeRoot 'thinkthen.exe'
    Write-PrivateFile $original ([System.IO.File]::ReadAllBytes($NativeBinary))
    $old = Get-FileHashPrivate $original
    $stage = Join-Path $script:Scratch 'stage.exe'
    Write-PrivateFile $stage ([System.IO.File]::ReadAllBytes($NativeBinary))
    Check ([ThinkThenInstall.Native]::Probe($stage) -cmatch ('\Athinkthen ' + [regex]::Escape($ReleaseVersion) + '(?:\r?\n)?\z')) 'real executable version probe'
    $snapshot = Save-Snapshot $original $old 'original.exe'
    # Fail with actual states that model ReplaceFile's documented partial errors.
    # The hook changes native names before throwing; it does not merely throw.
    function Invoke-FileReplace([string] $Stage, [string] $Destination, [string] $Backup) {
        if ($script:Fault -eq '1176') { throw [System.ComponentModel.Win32Exception]::new(1176) }
        if ($script:Fault -eq '1177') { [System.IO.File]::Move($Destination, $Backup); throw [System.ComponentModel.Win32Exception]::new(1177) }
        if ($script:Fault -eq 'after') { [System.IO.File]::Replace($Stage, $Destination, $Backup, $false); throw 'failure after replacement' }
        [System.IO.File]::Replace($Stage, $Destination, $Backup, $false)
    }
    foreach ($fault in @('1176', '1177', 'after')) {
        $script:Fault = $fault
        Refuses { Commit-File $stage $original $old } ('replacement exception ' + $fault)
        Check ($script:LastReplacementState.Count -eq 3) ('inspect all replacement names ' + $fault)
        if ($fault -eq '1177') { Check ($null -eq (Get-Attributes $original)) '1177 leaves destination absent' }
        $script:Fault = ''
        Restore-File $original $snapshot $old
        Check ((Get-FileHashPrivate $original) -ceq $old) ('verified restoration ' + $fault)
        if ($null -eq (Get-Attributes $stage)) { Write-PrivateFile $stage ([System.IO.File]::ReadAllBytes($NativeBinary)) }
    }
    $script:Fault = ''
    $locked = [System.IO.File]::Open($original, [System.IO.FileMode]::Open, [System.IO.FileAccess]::Read, [System.IO.FileShare]::Read)
    try { Refuses { Commit-File $stage $original $old } 'sharing violation preserves command' } finally { $locked.Dispose() }
    Check ((Get-FileHashPrivate $original) -ceq $old) 'sharing violation exact preservation'
    # Child file privacy must be checked even under a private directory. Check
    # the ACL and original bytes externally; installer never rewrites access.
    $acl = Get-Acl -LiteralPath $original
    $foreign = [System.Security.AccessControl.FileSystemAccessRule]::new([System.Security.Principal.SecurityIdentifier]::new('S-1-1-0'), 'Write', 'Allow')
    $acl.AddAccessRule($foreign); Set-Acl -LiteralPath $original -AclObject $acl
    $beforeAcl = (Get-Acl -LiteralPath $original).Sddl
    Refuses { Get-FileHashPrivate $original } 'foreign-write binary refuses before hashing'
    Check ((Get-Acl -LiteralPath $original).Sddl -ceq $beforeAcl) 'insecure binary ACL remains unchanged'
    Check ((Get-BytesHash ([System.IO.File]::ReadAllBytes($original))) -ceq $old) 'insecure binary bytes remain unchanged'
    $acl.RemoveAccessRuleSpecific($foreign); Set-Acl -LiteralPath $original -AclObject $acl
    $privateAcl = Get-Acl -LiteralPath $original
    $foreignOwnerAcl = Get-Acl -LiteralPath $original
    $foreignOwnerAcl.SetOwner([System.Security.Principal.SecurityIdentifier]::new('S-1-5-32-544'))
    Set-Acl -LiteralPath $original -AclObject $foreignOwnerAcl
    $ownerBefore = (Get-Acl -LiteralPath $original).Sddl
    Refuses { Get-FileHashPrivate $original } 'foreign-owner binary refuses before hashing'
    Check ((Get-Acl -LiteralPath $original).Sddl -ceq $ownerBefore) 'foreign binary owner remains unchanged'
    Set-Acl -LiteralPath $original -AclObject $privateAcl
    $unsafe = Join-Path $nativeRoot 'unsafe-target'
    New-PrivateDirectory $unsafe
    $unsafeAcl = Get-Acl -LiteralPath $unsafe; $unsafeAcl.AddAccessRule($foreign)
    Set-Acl -LiteralPath $unsafe -AclObject $unsafeAcl
    Refuses { Ensure-InstallDirectory $unsafe } 'existing insecure target refuses without ACL repair'
    $linkedParent = Join-Path $nativeRoot 'linked-parent'
    New-Item -ItemType Junction -Path $linkedParent -Target $unsafe | Out-Null
    Refuses { Get-InstallDirectory (Join-Path $linkedParent 'target') } 'junction ancestor refuses'
    [System.IO.Directory]::Delete($linkedParent, $false)
    foreach ($badPath in @('relative', '\\server\share\thinkthen', '\\?\C:\thinkthen', 'C:\thinkthen:stream')) {
        Refuses { Get-InstallDirectory $badPath } 'nonlocal or stream install path refuses'
    }
    $receipt = Join-Path $nativeRoot 'thinkthen.install.json'
    $pending = [ordered] @{ schema_version=1; state='pending'; executable_path=$original; version=$ReleaseVersion; old_sha256=$old; new_sha256=$old }
    Write-PrivateFile $receipt ([System.Text.Encoding]::UTF8.GetBytes(($pending | ConvertTo-Json -Compress)))
    $acl = Get-Acl -LiteralPath $receipt; $acl.AddAccessRule($foreign); Set-Acl -LiteralPath $receipt -AclObject $acl
    $receiptAcl = (Get-Acl -LiteralPath $receipt).Sddl
    Refuses { Read-Receipt $receipt $original } 'foreign-write receipt refuses before reading'
    Check ((Get-Acl -LiteralPath $receipt).Sddl -ceq $receiptAcl) 'insecure receipt ACL remains unchanged'
    $acl.RemoveAccessRuleSpecific($foreign); Set-Acl -LiteralPath $receipt -AclObject $acl
    $receiptPrivateAcl = Get-Acl -LiteralPath $receipt
    $foreignOwnerAcl = Get-Acl -LiteralPath $receipt
    $foreignOwnerAcl.SetOwner([System.Security.Principal.SecurityIdentifier]::new('S-1-5-32-544'))
    Set-Acl -LiteralPath $receipt -AclObject $foreignOwnerAcl
    Refuses { Read-Receipt $receipt $original } 'foreign-owner receipt refuses before reading'
    Set-Acl -LiteralPath $receipt -AclObject $receiptPrivateAcl
    Assert-ReceiptState $receipt $original $old $nativeRoot
    $absent = Join-Path $script:Scratch 'removed-original.exe'
    [System.IO.File]::Move($original, $absent); $script:Owned.Add($absent) | Out-Null
    Refuses { Assert-ReceiptState $receipt $original '' $nativeRoot } 'interrupted absent upgrade refuses before rollback'
    [System.IO.File]::Move($absent, $original)
    $removedReceipt = Join-Path $script:Scratch 'removed-receipt.json'
    [System.IO.File]::Move($receipt, $removedReceipt); $script:Owned.Add($removedReceipt) | Out-Null
    Refuses { Assert-ReceiptState $receipt $original $old $nativeRoot } 'missing receipt with earlier scratch refuses'
    [System.IO.File]::Move($removedReceipt, $receipt)
    # Actual child processes prove capture limits, timeout and environment scrub.
    $compiler = Join-Path $env:SystemRoot 'Microsoft.NET\Framework64\v4.0.30319\csc.exe'
    if (-not [System.IO.File]::Exists($compiler)) { throw 'Native version-probe fixtures require the installed .NET Framework compiler.' }
    foreach ($mode in @('good', 'wrong', 'error', 'exit', 'timeout', 'output')) {
        $source = Join-Path $script:Scratch ('probe-' + $mode + '.cs')
        $probe = Join-Path $script:Scratch ('probe-' + $mode + '.exe')
        $body = 'Console.WriteLine("thinkthen 0.2.0");'
        switch ($mode) {
            wrong { $body = 'Console.WriteLine("thinkthen 9.9.9");' }
            error { $body = 'Console.Error.WriteLine("unexpected");' }
            exit { $body = 'Environment.Exit(5);' }
            timeout { $body = 'System.Threading.Thread.Sleep(15000);' }
            output { $body = 'Console.Write(new string((char)120, 8192));' }
        }
        $code = 'using System; class Probe { static void Main() { foreach(string key in Environment.GetEnvironmentVariables().Keys) { if(key.StartsWith("THINKTHEN_", StringComparison.OrdinalIgnoreCase) || key.EndsWith("_API_KEY", StringComparison.OrdinalIgnoreCase)) { Console.Error.WriteLine("setting leaked"); return; } } ' + $body + ' } }'
        Write-PrivateFile $source ([System.Text.Encoding]::UTF8.GetBytes($code))
        $script:Owned.Add($probe) | Out-Null
        & $compiler /nologo /target:exe /platform:x64 ("/out:" + $probe) $source | Out-Null
        if ($LASTEXITCODE -ne 0) { throw 'Could not compile native version-probe fixture.' }
        $probeAcl = Get-Acl -LiteralPath $probe
        $probeAcl.SetOwner([System.Security.Principal.SecurityIdentifier]::new($script:UserSid))
        Set-Acl -LiteralPath $probe -AclObject $probeAcl
        Assert-Private $probe $false
        $env:THINKTHEN_PROBE_SENTINEL = 'fixture'; $env:FIXTURE_API_KEY = 'fixture'
        if ($mode -eq 'good') { Check ([ThinkThenInstall.Native]::Probe($probe) -ceq "thinkthen 0.2.0`r`n") 'version child strips settings and keys' }
        elseif ($mode -eq 'wrong') { Check ([ThinkThenInstall.Native]::Probe($probe) -cnotmatch '\Athinkthen 0\.2\.0(?:\r?\n)?\z') 'wrong version differs exactly' }
        else { Refuses { [ThinkThenInstall.Native]::Probe($probe) } ('version probe ' + $mode + ' refuses') }
        Remove-Item Env:THINKTHEN_PROBE_SENTINEL, Env:FIXTURE_API_KEY
    }
    $receiptBytes = [System.IO.File]::ReadAllBytes($receipt)
    foreach ($bad in @('not json', '{}', ($pending | ConvertTo-Json -Compress).Replace('"schema_version":1', '"schema_version":2'),
                       ($pending | ConvertTo-Json -Compress).Replace('"state":"pending"', '"state":"other"'),
                       ($pending | ConvertTo-Json -Compress).Replace($old, ('f' * 64)))) {
        [System.IO.File]::WriteAllText($receipt, $bad)
        Refuses { Assert-ReceiptState $receipt $original $old $nativeRoot } 'malformed or mismatched receipt refuses'
    }
    [System.IO.File]::WriteAllBytes($receipt, $receiptBytes)
    # Cleanup refuses an injected child before deleting any verified snapshot.
    $injected = Join-Path $script:Scratch 'unowned.txt'
    [System.IO.File]::WriteAllText($injected, 'unowned')
    Refuses { Remove-OwnedScratch $script:Scratch } 'cleanup refuses injected unowned child'
    Check ([System.IO.File]::Exists($snapshot)) 'cleanup preserves verified snapshot on refusal'
    [System.IO.File]::Delete($injected) # This fixture created this exact file.
    $junction = Join-Path $script:Scratch 'injected-junction'
    New-Item -ItemType Junction -Path $junction -Target $nativeRoot | Out-Null
    $script:Owned.Add($junction) | Out-Null
    Refuses { Remove-OwnedScratch $script:Scratch } 'cleanup refuses injected reparse child'
    Check ([System.IO.File]::Exists($snapshot)) 'reparse cleanup refusal retains snapshot'
    [System.IO.Directory]::Delete($junction, $false) # Remove this fixture junction only.
    Remove-OwnedScratch $script:Scratch
    Check (-not [System.IO.Directory]::Exists($script:Scratch)) 'owned flat cleanup succeeds'
    $env:THINKTHEN_INSTALL_DIR = $nativeRoot
    $nativeLock = Join-Path $nativeRoot '.thinkthen-install.lock'
    $held = [ThinkThenInstall.Native]::NewFile($nativeLock, $script:UserSid, $true)
    try { Refuses { Invoke-ThinkThenInstall $ReleaseVersion } 'concurrent installer refuses before receipt or command replacement' }
    finally { $held.Dispose() }
    Check ((Get-FileHashPrivate $original) -ceq $old) 'concurrent refusal preserves command'
    # Full installation drives the real downloaded executable, receipts, lock,
    # backups and current-run cleanup through the counted loopback release server.
    $env:THINKTHEN_INSTALL_BASE = $ReleaseBase
    $env:THINKTHEN_INSTALL_API = $ReleaseBase
    $pathBefore = $env:PATH
    $env:THINKTHEN_INSTALL_SENTINEL = 'must not reach version probe'
    $env:FAKE_SERVICE_API_KEY = 'fixture-sentinel'
    foreach ($scenario in @('first', 'upgrade', 'pending-old', 'pending-new', 'pending-absent-first',
                            'receipt-first-failure', 'binary-1177', 'installed-receipt-1177',
                            'after-binary', 'rollback-failure')) {
        $target = Join-Path $FixtureRoot ('installed space ' + [char] 0x3a9 + '-' + [guid]::NewGuid().ToString('D'))
        New-PrivateDirectory $target
        $env:THINKTHEN_INSTALL_DIR = $target
        $exe = Join-Path $target 'thinkthen.exe'
        $receiptName = Join-Path $target 'thinkthen.install.json'
        $script:Owned = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)
        $raw = [System.IO.File]::ReadAllBytes($NativeBinary)
        # A valid old command with distinct bytes proves byte preservation.
        $older = [byte[]]::new($raw.Length + 1)
        [Array]::Copy($raw, $older, $raw.Length)
        $older[$raw.Length] = 1
        $oldDigest = Get-BytesHash $older
        $newDigest = Get-BytesHash $raw
        $originalReceipt = ''
        if ($scenario -notin @('first', 'pending-absent-first', 'receipt-first-failure')) {
            Write-PrivateFile $exe $older
            $previous = [ordered] @{ schema_version=1; state='installed'; executable_path=$exe; version=$ReleaseVersion; old_sha256=''; new_sha256=$oldDigest }
            if ($scenario -in @('pending-old', 'pending-new')) {
                $previous.state = 'pending'; $previous.old_sha256 = $oldDigest; $previous.new_sha256 = $newDigest
                if ($scenario -eq 'pending-new') { [System.IO.File]::WriteAllBytes($exe, $raw) }
            }
            $originalReceipt = $previous | ConvertTo-Json -Compress
            Write-PrivateFile $receiptName ([System.Text.Encoding]::UTF8.GetBytes($originalReceipt))
        } elseif ($scenario -eq 'pending-absent-first') {
            $previous = [ordered] @{ schema_version=1; state='pending'; executable_path=$exe; version=$ReleaseVersion; old_sha256=''; new_sha256=$newDigest }
            Write-PrivateFile $receiptName ([System.Text.Encoding]::UTF8.GetBytes(($previous | ConvertTo-Json -Compress)))
        }
        $script:Scenario = $scenario; $script:ReplaceNumber = 0
        function Invoke-FileReplace([string] $Stage, [string] $Destination, [string] $Backup) {
            $script:ReplaceNumber++
            $file = [System.IO.Path]::GetFileName($Stage)
            if ($script:Scenario -eq 'binary-1177' -and $file -eq 'command.exe') {
                [System.IO.File]::Move($Destination, $Backup)
                throw [System.ComponentModel.Win32Exception]::new(1177)
            }
            if ($script:Scenario -eq 'installed-receipt-1177' -and $file -eq 'installed.json') {
                [System.IO.File]::Move($Destination, $Backup)
                throw [System.ComponentModel.Win32Exception]::new(1177)
            }
            if ($script:Scenario -eq 'rollback-failure' -and ($file -eq 'command.exe' -or $file.StartsWith('restore-'))) {
                [System.IO.File]::Replace($Stage, $Destination, $Backup, $false)
                throw 'primary planted replacement failure'
            }
            [System.IO.File]::Replace($Stage, $Destination, $Backup, $false)
            if ($script:Scenario -eq 'after-binary' -and $file -eq 'command.exe') { throw 'primary after-binary failure' }
        }
        # First installation uses nonoverwriting Move, so force preparation
        # refusal with an invalid checksum independently of Replace hooks.
        if ($scenario -eq 'receipt-first-failure') {
            $invalid = [ordered] @{ schema_version=1; state='pending'; executable_path=$exe; version=$ReleaseVersion; old_sha256=$oldDigest; new_sha256=$newDigest }
            Write-PrivateFile $receiptName ([System.Text.Encoding]::UTF8.GetBytes(($invalid | ConvertTo-Json -Compress)))
            Refuses { Invoke-ThinkThenInstall $ReleaseVersion } 'first receipt mismatch refuses'
            Check ($null -eq (Get-Attributes $exe)) 'first receipt refusal leaves command absent'
            continue
        }
        if ($scenario -in @('binary-1177', 'installed-receipt-1177', 'after-binary', 'rollback-failure')) {
            Refuses { Invoke-ThinkThenInstall $ReleaseVersion } ('full installer caught failure ' + $scenario)
            Check ((Get-FileHashPrivate $exe) -ceq $oldDigest) ('old bytes after caught failure ' + $scenario)
            if ($scenario -eq 'rollback-failure') {
                Check ([System.IO.Directory]::Exists($script:Scratch)) 'rollback exception retains current verified snapshots'
                Check ((Get-FileHashPrivate (Join-Path $script:Scratch 'original-command.exe')) -ceq $oldDigest) 'retained original command snapshot'
                Check ((Read-Receipt $receiptName $exe).state -ceq 'pending') 'failed rollback retains pending evidence'
            } else {
                Check ([System.IO.File]::ReadAllText($receiptName) -ceq $originalReceipt) ('original receipt bytes restored ' + $scenario)
                Check (-not [System.IO.Directory]::Exists($script:Scratch)) ('caught recovery cleans only owned scratch ' + $scenario)
            }
        } else {
            Invoke-ThinkThenInstall $ReleaseVersion
            Check ((Get-FileHashPrivate $exe) -ceq $newDigest) ('installed bytes ' + $scenario)
            Check ((Read-Receipt $receiptName $exe).state -ceq 'installed') ('installed receipt ' + $scenario)
            Check (@([System.IO.Directory]::EnumerateFileSystemEntries($target)).Count -eq 3) ('success leaves command receipt persistent lock ' + $scenario)
        }
        Check ($env:PATH -ceq $pathBefore) 'installer preserves PATH'
        Check ($env:THINKTHEN_INSTALL_SENTINEL -ceq 'must not reach version probe') 'installer preserves parent settings'
    }
    [Console]::WriteLine('Native Windows boundary proof completed for this host.')
} else { [Console]::WriteLine('Windows installer execution, NTFS, ACL and sharing proof NOT RUN; portable boundaries only.') }
[Console]::WriteLine('PowerShell {0}: {1} boundary checks passed.', $PSVersionTable.PSVersion, $script:Checks)
