# Language: en-GB
# Synthetic files only; no Rust build, installation or user data is used.
$ErrorActionPreference = 'Stop'
$projectRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
$fixture = Join-Path $projectRoot ('.build\build-output-tests\' + [guid]::NewGuid().ToString('N'))
$output = Join-Path $fixture 'output with spaces'
New-Item -ItemType Directory -Path $output -Force | Out-Null
$source = Join-Path $fixture 'synthetic.exe'
$writer = Join-Path $PSScriptRoot 'Write-BuildOutput.ps1'
$executable = Join-Path $output 'MonitorSwitcher.exe'
$checksum = Join-Path $output 'SHA256SUMS.txt'
$encoding = [Text.UTF8Encoding]::new($false)

function Assert-Output([string]$ExpectedHash) {
    if ((Get-FileHash -LiteralPath $executable -Algorithm SHA256).Hash -ne $ExpectedHash) {
        throw 'The packaged executable does not match the synthetic input.'
    }
    $contents = [IO.File]::ReadAllText($checksum)
    if ($contents -cne "# Language: en-GB`n$ExpectedHash  MonitorSwitcher.exe`n") {
        throw 'The generated checksum or language metadata does not match the executable.'
    }
}

[IO.File]::WriteAllBytes($source, [byte[]](1, 2, 3, 4))
[IO.File]::WriteAllBytes($executable, [byte[]](9, 9))
[IO.File]::WriteAllText($checksum, 'stale checksum', $encoding)
$firstHash = (Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash
& $writer -SourceExecutable $source -OutputDirectory $output | Out-Null
Assert-Output $firstHash

[IO.File]::WriteAllBytes($source, [byte[]](5, 6, 7, 8))
$secondHash = (Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash
if ($secondHash -eq $firstHash) { throw 'Synthetic inputs unexpectedly have equal hashes.' }
& $writer -SourceExecutable $source -OutputDirectory $output | Out-Null
Assert-Output $secondHash

$checksumBytes = [IO.File]::ReadAllBytes($checksum)
if ($checksumBytes.Length -ge 3 -and $checksumBytes[0] -eq 0xEF -and $checksumBytes[1] -eq 0xBB -and $checksumBytes[2] -eq 0xBF) {
    throw 'Checksum output unexpectedly contains a UTF-8 byte-order mark.'
}
$before = [IO.File]::ReadAllText($checksum)
$rejected = $false
try { & $writer -SourceExecutable (Join-Path $fixture 'missing.exe') -OutputDirectory $output | Out-Null }
catch { $rejected = $true }
if (-not $rejected) { throw 'Missing executable was accepted.' }
if ([IO.File]::ReadAllText($checksum) -cne $before) { throw 'Missing input changed the checksum.' }
Assert-Output $secondHash
[pscustomobject]@{ BuildOutputRegressionChecks = 5; Result = 'PASS'; UsesSyntheticFixtures = $true }
