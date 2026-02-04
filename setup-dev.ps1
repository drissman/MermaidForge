# MermaidForge - Quick Setup Script for Windows
# Run this in PowerShell: .\setup-dev.ps1

Write-Host "🎨 MermaidForge - Development Setup" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "⚠️  Warning: Not running as Administrator" -ForegroundColor Yellow
    Write-Host "Some operations may fail. Consider running as Admin." -ForegroundColor Yellow
    Write-Host ""
}

# Check prerequisites
Write-Host "📋 Checking prerequisites..." -ForegroundColor Green

# Check Node.js
$nodeVersion = node --version 2>$null
if ($nodeVersion) {
    Write-Host "✅ Node.js installed: $nodeVersion" -ForegroundColor Green
} else {
    Write-Host "❌ Node.js not found!" -ForegroundColor Red
    Write-Host "   Download from: https://nodejs.org" -ForegroundColor Yellow
    exit 1
}

# Check Rust
$rustVersion = rustc --version 2>$null
if ($rustVersion) {
    Write-Host "✅ Rust installed: $rustVersion" -ForegroundColor Green
} else {
    Write-Host "❌ Rust not found!" -ForegroundColor Red
    Write-Host "   Download from: https://rustup.rs" -ForegroundColor Yellow
    exit 1
}

# Check Git
$gitVersion = git --version 2>$null
if ($gitVersion) {
    Write-Host "✅ Git installed: $gitVersion" -ForegroundColor Green
} else {
    Write-Host "❌ Git not found!" -ForegroundColor Red
    Write-Host "   Download from: https://git-scm.com" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "📦 Installing dependencies..." -ForegroundColor Green

# Install npm dependencies
Write-Host "   Installing Node packages..." -ForegroundColor Cyan
npm install
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ npm install failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Node packages installed" -ForegroundColor Green

# Build Rust dependencies (check)
Write-Host "   Checking Rust dependencies..." -ForegroundColor Cyan
Set-Location src-tauri
cargo check
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Cargo check failed!" -ForegroundColor Red
    Set-Location ..
    exit 1
}
Write-Host "✅ Rust dependencies OK" -ForegroundColor Green
Set-Location ..

Write-Host ""
Write-Host "🎉 Setup completed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "📝 Next steps:" -ForegroundColor Cyan
Write-Host "   1. Start dev server:  npm run tauri dev" -ForegroundColor White
Write-Host "   2. Build portable:    npm run tauri:build:portable" -ForegroundColor White
Write-Host "   3. Build installer:   npm run tauri:build:msi" -ForegroundColor White
Write-Host ""
Write-Host "📚 Documentation:" -ForegroundColor Cyan
Write-Host "   README.md        - General documentation" -ForegroundColor White
Write-Host "   GITHUB_GUIDE.md  - Git/GitHub guide for beginners" -ForegroundColor White
Write-Host "   CONTRIBUTING.md  - How to contribute" -ForegroundColor White
Write-Host ""
Write-Host "🚀 Happy coding!" -ForegroundColor Green
