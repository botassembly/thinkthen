# Runs the shown program from this folder, as run.sh files run show.sh.
# --check gives 0 for yes and 1 for no, and show.ps1 branches on both.
# Any other code is a failure, so pass it on for the capture to record.
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
Set-Location $PSScriptRoot
& ./show.ps1
if ($LASTEXITCODE -ge 2) { exit $LASTEXITCODE }
exit 0
