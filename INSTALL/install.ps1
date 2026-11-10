# Install squeeze from the latest GitHub release (Windows).
#   irm https://raw.githubusercontent.com/jongwooish/claude-squeeze/main/INSTALL/install.ps1 | iex
# Pin a version:  $env:SQUEEZE_VERSION = "v0.2.0"; irm ... | iex
# Prefer a Start menu icon? Use squeeze-Auditor-<version>-setup-x64.exe from the Releases page.
$ErrorActionPreference = "Stop"

$Repo = "jongwooish/claude-squeeze"
$Version = if ($env:SQUEEZE_VERSION) { $env:SQUEEZE_VERSION } else { "latest" }
$Dir = if ($env:SQUEEZE_INSTALL_DIR) { $env:SQUEEZE_INSTALL_DIR } else { Join-Path $env:LOCALAPPDATA "Programs\squeeze" }

$arch = if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") { "aarch64" } else { "x86_64" }
$asset = "squeeze-$arch-pc-windows-msvc.zip"
$base = if ($Version -eq "latest") { "https://github.com/$Repo/releases/latest/download" } else { "https://github.com/$Repo/releases/download/$Version" }

$tmp = Join-Path ([IO.Path]::GetTempPath()) ("squeeze-" + [guid]::NewGuid())
New-Item -ItemType Directory -Path $tmp | Out-Null
try {
    Write-Host "squeeze-install: downloading $asset ($Version)"
    try {
        Invoke-WebRequest "$base/$asset" -OutFile (Join-Path $tmp $asset) -UseBasicParsing
        Invoke-WebRequest "$base/checksums.txt" -OutFile (Join-Path $tmp "checksums.txt") -UseBasicParsing
    } catch {
        Write-Host "squeeze-install: could not download $asset ($Version)." -ForegroundColor Yellow
        Write-Host "squeeze-install: check https://github.com/$Repo/releases for available versions," -ForegroundColor Yellow
        Write-Host "squeeze-install: or build from source: cargo install --git https://github.com/$Repo squeeze" -ForegroundColor Yellow
        return
    }

    $line = Get-Content (Join-Path $tmp "checksums.txt") | Where-Object { $_ -match "\s$([regex]::Escape($asset))$" }
    if (-not $line) { throw "$asset not listed in checksums.txt" }
    $expected = ($line -split "\s+")[0].ToLower()
    $actual = (Get-FileHash (Join-Path $tmp $asset) -Algorithm SHA256).Hash.ToLower()
    if ($expected -ne $actual) { throw "checksum mismatch for $asset" }
    Write-Host "squeeze-install: checksum ok"

    Expand-Archive (Join-Path $tmp $asset) -DestinationPath $tmp -Force
    New-Item -ItemType Directory -Path $Dir -Force | Out-Null
    Copy-Item (Join-Path $tmp "squeeze.exe") (Join-Path $Dir "squeeze.exe") -Force
    $auditor = Join-Path $tmp "squeeze-auditor.exe"
    if (Test-Path $auditor) { Copy-Item $auditor (Join-Path $Dir "squeeze-auditor.exe") -Force }

    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if (-not ($userPath -split ";" | Where-Object { $_ -eq $Dir })) {
        [Environment]::SetEnvironmentVariable("Path", "$userPath;$Dir", "User")
        Write-Host "squeeze-install: added $Dir to your user PATH (open a new terminal)"
    }
    Write-Host "squeeze-install: installed to $Dir\squeeze.exe"
    if (Test-Path $auditor) {
        Write-Host "squeeze-install: next: squeeze setup   (connect Claude Code for all projects)"
        Write-Host "squeeze-install:       squeeze-auditor (open the dashboard)"
        Write-Host "squeeze-install: tip: the Setup installer on the Releases page also adds a Start menu icon"
    } else {
        Write-Host "squeeze-install: next: cd your-project; squeeze init"
    }
} finally {
    Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue
}
