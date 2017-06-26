$outputExe = "$(Get-Location)\target\debug\rust-examples.exe"

cargo clean ; cargo build
if (Test-Path $outputExe) {
    Write-Host "Executing..." -ForegroundColor Green
    .\target\debug\rust-examples.exe
} else {
    Write-Host "Failed!" -ForegroundColor Red
}