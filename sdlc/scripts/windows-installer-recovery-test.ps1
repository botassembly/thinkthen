# Execute the real transaction and cleanup ASTs against failing I/O boundaries.
# This proves error preservation, not native Windows filesystem behavior.
param([string] $Installer, [ValidateSet('transaction', 'replacement', 'committed', 'uncommitted')] [string] $Mode = 'transaction', [string] $FixtureRoot = '')
Set-StrictMode -Version 2.0
$ErrorActionPreference = 'Stop'
$phaseWriter = [Console]::Error
$phaseWriter.WriteLine('Recovery fixture phase: parse installer')
$tokens = $null; $errors = $null
$ast = [System.Management.Automation.Language.Parser]::ParseFile($Installer, [ref] $tokens, [ref] $errors)
if ($errors.Count) { throw 'Installer parse failed.' }
$phaseWriter.WriteLine('Recovery fixture phase: find transaction and cleanup ASTs')
$transaction = @($ast.FindAll({
    param($node)
    if ($node -isnot [System.Management.Automation.Language.TryStatementAst]) { return $false }
    $first = $node.Body.Statements[0]
    if ($first -isnot [System.Management.Automation.Language.PipelineAst]) { return $false }
    $command = $first.PipelineElements[0]
    return $command -is [System.Management.Automation.Language.CommandAst] -and
        $command.GetCommandName() -ceq 'Commit-File' -and
        $command.CommandElements[1].Extent.Text -ceq '$pendingStage'
}, $true))
$cleanup = @($ast.FindAll({
    param($node)
    return $node -is [System.Management.Automation.Language.TryStatementAst] -and
        $null -ne $node.Finally -and $node.Finally.Extent.Text.Contains('Remove-OwnedScratch')
}, $true))
if ($transaction.Count -ne 1 -or $cleanup.Count -ne 1) { throw 'Expected one installer transaction and cleanup block.' }
function New-FailingErrorWriter {
    # A closed real writer fails every WriteLine overload without starting a
    # compiler in each short-lived recovery process on a cold Windows host.
    $writer = [System.IO.StringWriter]::new()
    $writer.Dispose()
    return $writer
}
if ($Mode -in @('committed', 'uncommitted')) {
    $phaseWriter.WriteLine('Recovery fixture phase: find entry point AST')
    $entry = @($ast.EndBlock.Statements | Where-Object { $_ -is [System.Management.Automation.Language.TryStatementAst] })
    if ($entry.Count -ne 1) { throw 'Expected the actual installer entry point.' }
    function Invoke-ThinkThenInstall {
        $script:Committed = $Mode -eq 'committed'
        throw 'OUTPUT ERROR'
    }
    $Version = '0.2.0'
    [Console]::SetError((New-FailingErrorWriter))
    $phaseWriter.WriteLine('Recovery fixture phase: execute entry point AST')
    & ([scriptblock]::Create($entry[0].Extent.Text))
    return
}
if ($Mode -eq 'replacement') {
    $phaseWriter.WriteLine('Recovery fixture phase: load replacement function ASTs')
    foreach ($name in @('Commit-File', 'Restore-File', 'Get-ReplacementState')) {
        $function = @($ast.EndBlock.Statements | Where-Object {
            $_ -is [System.Management.Automation.Language.FunctionDefinitionAst] -and $_.Name -ceq $name
        })
        if ($function.Count -ne 1) { throw ('Expected the actual installer function: ' + $name) }
        . ([scriptblock]::Create($function[0].Extent.Text))
    }
    function Assert-Ancestors {}
    function Assert-Private {}
    function Get-Attributes { return $null }
    function Get-CurrentDigest {
        if ($script:Replacing) { throw 'INSPECTION ERROR' }
        return 'old'
    }
    function Get-FileHashPrivate { return 'original' }
    function Write-PrivateFile {}
    function Invoke-FileReplace { $script:Replacing = $true; throw 'PRIMARY REPLACEMENT ERROR' }
    $script:InstallDirectory = $FixtureRoot; $script:Scratch = $FixtureRoot
    $script:Owned = [System.Collections.Generic.HashSet[string]]::new()
    $phaseWriter.WriteLine('Recovery fixture phase: create replacement snapshot')
    $snapshot = Join-Path $FixtureRoot ('replacement-snapshot-' + [guid]::NewGuid().ToString('D'))
    $file = [System.IO.File]::Open($snapshot, [System.IO.FileMode]::CreateNew, [System.IO.FileAccess]::Write, [System.IO.FileShare]::None)
    try { $file.WriteByte(1) } finally { $file.Dispose() }
    $priorWriter = [Console]::Error
    $writer = New-FailingErrorWriter
    [Console]::SetError($writer)
    try {
        foreach ($operation in @('commit', 'restore')) {
            $phaseWriter.WriteLine('Recovery fixture phase: replacement ' + $operation)
            $script:Replacing = $false; $script:LastReplacementState = @{}
            $actual = ''
            try {
                if ($operation -eq 'commit') { Commit-File 'stage' 'destination' 'old' }
                else { Restore-File 'destination' $snapshot 'original' }
            } catch { $actual = $_.Exception.Message }
            if ($actual -cne 'PRIMARY REPLACEMENT ERROR') { throw ('Expected primary replacement error; returned: ' + $actual) }
            if ($script:LastReplacementState.Count -ne 3 -or
                @($script:LastReplacementState.Values | Where-Object { $_ -cne 'uninspectable' }).Count -ne 0) {
                throw 'Replacement inspection did not retain all three uninspectable states.'
            }
        }
    } finally {
        [Console]::SetError($priorWriter); $writer.Dispose()
        [System.IO.File]::Delete($snapshot) # This fixture created this exact file.
    }
    [Console]::WriteLine('Actual Commit-File and Restore-File ASTs preserve the primary error and all 3 inspection states despite a failing error stream; native Windows proof NOT RUN.')
    return
}
$phaseWriter.WriteLine('Recovery fixture phase: prepare transaction cases')
function Commit-File { throw 'PRIMARY COMMIT ERROR' }
function Restore-File { throw 'ROLLBACK ERROR' }
function Write-PrivateFile {}
function Get-CurrentDigest { return '' }
function Get-Attributes {
    if ($script:InspectionFault) { throw 'INSPECTION ERROR' }
    return $null
}
function Remove-OwnedScratch { $script:CleanupCalls++ }
$script:Scratch = 'retained-scratch'
$pendingStage = 'pending'; $receiptPath = 'receipt'; $oldReceiptHash = 'old'
$stage = 'stage'; $installed = 'command'; $oldHash = 'old'; $newHash = 'new'
$installedStage = 'installed'; $pendingHash = 'pending'
$oldSnapshot = 'snapshot'; $receiptSnapshot = 'receipt snapshot'; $RequestedVersion = '0.2.0'
$lock = $null
$originalErrorWriter = [Console]::Error
foreach ($scenario in @('inspection-failure', 'absent-receipt', 'diagnostic-failure')) {
    $phaseWriter.WriteLine('Recovery fixture phase: transaction ' + $scenario)
    $retain = $false; $script:CleanupCalls = 0
    $script:InspectionFault = $scenario -ne 'absent-receipt'
    $capture = [System.IO.StringWriter]::new()
    $writer = $capture
    if ($scenario -eq 'diagnostic-failure') { $writer = New-FailingErrorWriter }
    [Console]::SetError($writer)
    try {
        $actual = ''
        try { . ([scriptblock]::Create($transaction[0].Extent.Text)) }
        catch { $actual = $_.Exception.Message }
        if ($actual -cne 'PRIMARY COMMIT ERROR') { throw ('Expected primary commit error; returned: ' + $actual) }
        if (-not $retain) { throw 'Failed rollback did not retain recovery artifacts.' }
        $phaseWriter.WriteLine('Recovery fixture phase: retained-artifact cleanup ' + $scenario)
        $body = $cleanup[0].Finally.Extent.Text
        . ([scriptblock]::Create($body.Substring(1, $body.Length - 2)))
        if ($script:CleanupCalls -ne 0) { throw 'Actual cleanup tried to remove retained recovery artifacts.' }
        $phaseWriter.WriteLine('Recovery fixture phase: verify diagnostics ' + $scenario)
        if ($scenario -ne 'diagnostic-failure') {
            $diagnostics = $capture.ToString()
            $required = if ($script:InspectionFault) { 'Receipt inspection failed. Manual recovery is required: INSPECTION ERROR' }
                        else { 'Receipt is absent. Manual recovery is required.' }
            if (-not $diagnostics.Contains($required) -or -not $diagnostics.Contains('Retained recovery artifacts: retained-scratch')) {
                throw 'Recovery diagnostics did not report the inspected state and retained path.'
            }
        }
    } finally { [Console]::SetError($originalErrorWriter); $writer.Dispose(); if ($writer -ne $capture) { $capture.Dispose() } }
}
[Console]::WriteLine('Actual installer AST: 3 recovery cases preserve the primary error and skip retained-artifact cleanup; native Windows proof NOT RUN.')
