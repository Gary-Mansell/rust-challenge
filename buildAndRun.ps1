param (
    [switch]$clean = $false,
    [switch]$execute = $false,
    [switch]$docker = $false,
    [string]$outputExe = "$(Get-Location)\target\debug\rust-challenge.exe"
)

if ($clean) {
    Write-Host "Cleaning..." -ForegroundColor Green
    cargo clean
} 

Write-Host "Building..." -ForegroundColor Green
rustup run nightly cargo build

if ($execute) {
    Write-Host "Executing..." -ForegroundColor Green
    if (Test-Path $outputExe) {        
        cargo run
    } else {
        Write-Host "Failed!" -ForegroundColor Red
    }
}