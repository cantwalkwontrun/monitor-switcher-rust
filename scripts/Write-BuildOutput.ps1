# Language: en-GB
param(
    [Parameter(Mandatory)][string]$SourceExecutable,
    [Parameter(Mandatory)][string]$OutputDirectory
)
$ErrorActionPreference = 'Stop'
if (-not (Test-Path -LiteralPath $SourceExecutable -PathType Leaf)) {
    throw 'The built executable is missing; output files were not changed.'
}
New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
$executable = Join-Path $OutputDirectory 'MonitorSwitcher.exe'
Copy-Item -LiteralPath $SourceExecutable -Destination $executable
$hash = Get-FileHash -LiteralPath $executable -Algorithm SHA256
$manifest = "# Language: en-GB`n$($hash.Hash)  MonitorSwitcher.exe`n"
[IO.File]::WriteAllText((Join-Path $OutputDirectory 'SHA256SUMS.txt'), $manifest, [Text.UTF8Encoding]::new($false))
$hash
