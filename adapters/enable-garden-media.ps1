# Upgrade one Windows Garden and enable reviewed media for its harnesses.
[CmdletBinding()]
param([string]$MotherRoot = (Join-Path $env:LOCALAPPDATA 'MOTHER'), [string]$Config, [switch]$CheckOnly)
$ErrorActionPreference = 'Stop'
$repository = Split-Path $PSScriptRoot -Parent
$MotherRoot = (Resolve-Path -LiteralPath $MotherRoot).ProviderPath
$runtime = Join-Path $MotherRoot 'native-runtime'
$installed = Join-Path $MotherRoot 'bin\mother.exe'
$clients = Join-Path $MotherRoot 'clients'
$db = Join-Path $MotherRoot 'garden.redb'
$endpoint = Join-Path $runtime 'service.json'
$pidFile = Join-Path $runtime 'owner.pid'
$launcher = Join-Path $runtime 'Start-MotherAppTunnel.ps1'
foreach ($file in @($installed, $db, $endpoint)) {
    if (!(Test-Path -LiteralPath $file -PathType Leaf)) { throw "Missing existing installation file $file" }
}
function Require-Success([string]$action) {
    if ($LASTEXITCODE -ne 0) { throw "$action failed with exit code $LASTEXITCODE." }
}
# Match the launcher's configured alias and client locally without evaluating the script.
$tunnel = $null
if (Test-Path -LiteralPath $launcher -PathType Leaf) {
    $launchText = [IO.File]::ReadAllText($launcher)
    $aliasMatch = [regex]::Match($launchText, '(?m)^\s*\$alias\s*=\s*''([A-Za-z0-9_-]+)''\s*$')
    $clientMatch = [regex]::Match($launchText, '(?m)^\s*\$client\s*=\s*Join-Path \$motherRoot ''(tools\\tunnel-client\\[0-9.]+\\tunnel-client\.exe)''\s*$')
    if (!$aliasMatch.Success -or !$clientMatch.Success) { throw 'Cannot identify the existing tunnel launcher configuration.' }
    $tunnelAlias = $aliasMatch.Groups[1].Value
    $tunnel = Join-Path $MotherRoot $clientMatch.Groups[1].Value
    if (!(Test-Path -LiteralPath $tunnel -PathType Leaf)) { throw 'Existing tunnel client is missing.' }
}
# Capture the actual startup arguments, including Explorer and recall profiles.
function Has-Argument([string]$command, [string]$option, [string]$value) {
    $escaped = [regex]::Escape($value)
    $command -match ("(?:^|\s)" + $option + "\s+(?:\x22" + $escaped + "\x22|" + $escaped + ")(?=\s|$)")
}
$owners = @(Get-CimInstance Win32_Process -Filter "Name = 'mother.exe'" | Where-Object {
    $_.CommandLine -match '(?:^|\s)garden\s+service\s' -and
    (Has-Argument $_.CommandLine '--db' $db) -and (Has-Argument $_.CommandLine '--endpoint' $endpoint)
})
if ($owners.Count -ne 1) { throw 'Could not identify exactly one running owner of the configured Garden. No processes were stopped.' }
$owner = $owners[0]
$oldExe = $owner.ExecutablePath
if (!$oldExe -or !(Test-Path -LiteralPath $oldExe -PathType Leaf)) { throw 'Cannot identify the running owner executable.' }
$argsMatch = [regex]::Match($owner.CommandLine, '(?is)^(?:"[^"]+"|\S+)\s+(garden\s+service\s+.+)$')
if (!$argsMatch.Success) { throw 'Cannot preserve the existing owner launch arguments.' }
$ownerArgs = $argsMatch.Groups[1].Value
$serviceBefore = Get-Content -LiteralPath $endpoint -Raw | ConvertFrom-Json
if (!$serviceBefore.workspaceId -or !$serviceBefore.generation) {throw 'Missing existing Garden identity.'}
$configs = @()
if ($Config) {
    $configs = @((Get-Item -LiteralPath $Config))
} else {
    $configs = @(Get-ChildItem -LiteralPath $clients -Filter '*.json' -File | Where-Object {
        try {
            $profile = Get-Content -LiteralPath $_.FullName -Raw | ConvertFrom-Json
            $profile.backend -eq 'garden_service' -and $profile.workspaceId -eq $serviceBefore.workspaceId -and $profile.generation -eq $serviceBefore.generation -and $profile.endpointPath -eq $endpoint
        } catch { $false }
    })
}
if (!$configs.Count) { throw 'No existing service credential for this Garden. Supply its path with -Config.' }
$runtimeStatus = $null
if ($tunnel) {
    $runtimeStatus = & $tunnel runtimes status $tunnelAlias --json | ConvertFrom-Json
    Require-Success 'Inspect tunnel'
}
if ($CheckOnly) { Write-Output "Located the Garden owner and $($configs.Count) matching service configurations."; return }

Push-Location $repository
try { & cargo build --release --locked; Require-Success 'Build Mother' } finally { Pop-Location }
$source = Join-Path $repository 'target\release\mother.exe'
& $source garden media bridge --help | Out-Null
Require-Success 'Check built bridge'
$staged = Join-Path (Split-Path $installed) ('mother-stage-' + [guid]::NewGuid() + '.exe')
$backup = Join-Path (Split-Path $installed) ('mother-backup-' + [guid]::NewGuid() + '.exe')
Copy-Item -LiteralPath $source -Destination $staged
if ((Get-FileHash -LiteralPath $staged).Hash -ne (Get-FileHash -LiteralPath $source).Hash) { throw 'Staged binary hash mismatch.' }

function Start-Owner([string]$program) {
    $env:MOTHER_HOME = $MotherRoot
    $proc = Start-Process -FilePath $program -ArgumentList $ownerArgs -WorkingDirectory $MotherRoot -WindowStyle Hidden -PassThru
    [IO.File]::WriteAllText($pidFile, $proc.Id.ToString())
    return $proc
}
function Wait-Owner($proc) {
    for ($i = 0; $i -lt 80; $i++) {
        if ($proc.HasExited) { throw 'Replacement owner exited during startup.' }
        try {
            $info = Get-Content -LiteralPath $endpoint -Raw | ConvertFrom-Json
            if ($info.serviceId -ne $serviceBefore.serviceId -and $info.workspaceId -eq $serviceBefore.workspaceId -and $info.generation -eq $serviceBefore.generation) {return}
        } catch { }
        Start-Sleep -Milliseconds 250
    }
    throw 'Replacement owner did not publish the expected Garden identity.'
}
function Connect-Tunnel {
    & $launcher | Out-Null
    for ($i = 0; $i -lt 30; $i++) {
        $status = & $tunnel runtimes status $tunnelAlias --json | ConvertFrom-Json
        if ($LASTEXITCODE -eq 0 -and $status.ready -and $status.healthy) {return}
        Start-Sleep -Seconds 1
    }
    throw 'ChatGPT tunnel readiness could not be verified.'
}
$stopped = $false
$replaced = $false
$tunnelStopped = $false
$newOwner = $null
try {
    if ($runtimeStatus.process_running) {
        & $tunnel runtimes stop $tunnelAlias --json | Out-Null
        Require-Success 'Stop existing ChatGPT tunnel'
        $tunnelStopped = $true
    }
    $oldProcess = Get-Process -Id $owner.ProcessId
    if ($oldProcess.Path -ne $oldExe -or [math]::Abs(($oldProcess.StartTime - $owner.CreationDate).TotalSeconds) -gt 2) {
        throw 'Owner process identity changed during preparation.'
    }
    $oldProcess.Kill()
    $stopped = $true
    if (!$oldProcess.WaitForExit(10000)) {throw 'Original owner did not exit.'}
    [IO.File]::Replace($staged, $installed, $backup)
    $replaced = $true
    $newOwner = Start-Owner $installed
    Wait-Owner $newOwner
    $activated = $false
    foreach ($harness in $configs) {
        $null = & $installed garden media bridge --config $harness.FullName enable 2>$null
        if ($LASTEXITCODE -eq 0) { $activated = $true; break }
    }
    if (!$activated) { throw 'No configured credential could verify the shared Garden media setting.' }
    foreach ($harness in $configs) {
        $probe = & $installed garden media bridge --config $harness.FullName policy 2>$null
        if ($LASTEXITCODE -eq 0 -and ($probe | ConvertFrom-Json).available) {
            Write-Output ("Verified media access through " + $harness.Name)
        } else { Write-Warning ("Could not verify this credential: " + $harness.Name) }
    }
    if ($tunnelStopped) { Connect-Tunnel }
    Write-Output 'Garden identity verified. Reviewed media proposals are enabled for all authorized harnesses.'
} catch {
    $failure = $_
    if ($newOwner -and !$newOwner.HasExited) { $newOwner.Kill(); $newOwner.WaitForExit() }
    if ($replaced) { [IO.File]::Replace($backup, $installed, $staged) }
    if ($stopped) { $restored = Start-Owner $oldExe; Wait-Owner $restored }
    if ($tunnelStopped) { Connect-Tunnel }
    throw $failure
} finally {
    if (Test-Path -LiteralPath $staged -PathType Leaf) { Remove-Item -LiteralPath $staged }
}
