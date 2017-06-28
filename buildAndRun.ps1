param (
    [switch]$clean = $false,
    [switch]$execute = $false,
    [string]$outputExe = "$(Get-Location)\target\debug\rust-challenge.exe"
)

if ($clean) {
    Write-Host "Cleaning..." -ForegroundColor Green
    cargo clean
} 

Write-Host "Building..." -ForegroundColor Green
cargo build

if ($execute) {
    Write-Host "Executing..." -ForegroundColor Green
    if (Test-Path $outputExe) {        
        cargo run
    } else {
        Write-Host "Failed!" -ForegroundColor Red
    }
}