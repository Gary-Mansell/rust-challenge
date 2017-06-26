$outputExe = "$(Get-Location)\target\debug\rust-challenge.exe"

cargo clean ; cargo build
if (Test-Path $outputExe) {
    Write-Host "Executing..." -ForegroundColor Green
    .\target\debug\rust-challenge.exe
} else {
    Write-Host "Failed!" -ForegroundColor Red
}