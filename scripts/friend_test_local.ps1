$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $RepoRoot

$script:TotalSteps = 8
$script:CurrentStep = 0

function Write-FriendTestInfo {
    param([Parameter(Mandatory = $true)][string]$Message)
    Write-Host "[friend-test] $Message"
}

function Invoke-FriendTestStep {
    param(
        [Parameter(Mandatory = $true)][string]$Label,
        [Parameter(Mandatory = $true)][scriptblock]$Command
    )

    $script:CurrentStep += 1
    Write-FriendTestInfo "step $($script:CurrentStep)/$($script:TotalSteps): $Label"
    & $Command
    if ($LASTEXITCODE -ne $null -and $LASTEXITCODE -ne 0) {
        throw "Friend-test step failed with exit code ${LASTEXITCODE}: $Label"
    }
    $global:LASTEXITCODE = 0
}

Write-FriendTestInfo "AeroCodex local friend-test package"
Write-FriendTestInfo "repository root: $RepoRoot"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-FriendTestInfo "ERROR: cargo was not found on the command search path"
    Write-FriendTestInfo "Install Rust with cargo, rustfmt, and clippy before running the friend-test package."
    exit 127
}

if (Get-Command rustc -ErrorAction SilentlyContinue) {
    Write-FriendTestInfo "rustc: $(& rustc --version)"
} else {
    Write-FriendTestInfo "rustc: not found on the command search path"
}
Write-FriendTestInfo "cargo: $(& cargo --version)"

if (Get-Command git -ErrorAction SilentlyContinue) {
    & git rev-parse --is-inside-work-tree *> $null
    if ($LASTEXITCODE -eq 0) {
        Write-FriendTestInfo "git commit: $(& git log -1 --format=%h)"
    }
    $global:LASTEXITCODE = 0
}

Invoke-FriendTestStep "cargo fmt --all -- --check" {
    cargo fmt --all -- --check
}
Invoke-FriendTestStep "cargo clippy --workspace --all-targets --all-features -- -D warnings" {
    cargo clippy --workspace --all-targets --all-features -- -D warnings
}
Invoke-FriendTestStep "cargo test --workspace --all-features" {
    cargo test --workspace --all-features
}
Invoke-FriendTestStep "cargo run -p xtask -- verify --all" {
    cargo run -p xtask -- verify --all
}
Invoke-FriendTestStep "cargo run -p xtask -- verify equation-inventory" {
    cargo run -p xtask -- verify equation-inventory
}
Invoke-FriendTestStep "cargo run -p xtask -- verify formula-vault" {
    cargo run -p xtask -- verify formula-vault
}
Invoke-FriendTestStep "cargo run -p xtask -- dependency-policy" {
    cargo run -p xtask -- dependency-policy
}
Invoke-FriendTestStep "cargo doc --workspace --all-features --no-deps" {
    cargo doc --workspace --all-features --no-deps
}

if (Test-Path (Join-Path $RepoRoot "Cargo.lock")) {
    Write-FriendTestInfo "NOTE: a root Cargo.lock exists after the run. Do not submit it unless project policy changes."
}

Write-FriendTestInfo "completed all requested local checks"
Write-FriendTestInfo "Reminder: passing local checks does not prove physical validity, safety, certification, mission readiness, habitat safety, medical suitability, or regulated-use approval."
