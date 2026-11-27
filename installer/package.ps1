param(
    [string]$MakeNsis = 'makensis.exe',
    [switch]$Offline
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$manifest = Join-Path $root 'Cargo.toml'
$compiler = (Get-Command $MakeNsis -CommandType Application -ErrorAction Stop).Source

$cargoMetadata = cargo metadata --format-version 1 --no-deps --locked --manifest-path $manifest
if ($LASTEXITCODE -ne 0) { throw 'cargo metadata failed' }
$version = ($cargoMetadata | ConvertFrom-Json).packages[0].version

$target = 'x86_64-pc-windows-msvc'
$settings = 'target.x86_64-pc-windows-msvc.rustflags=["-C", "target-feature=+crt-static"]'
$buildArguments = @('build', '--release', '--locked', '--target', $target, '--config', $settings, '--manifest-path', $manifest)
if ($Offline) { $buildArguments += '--offline' }
cargo @buildArguments
if ($LASTEXITCODE -ne 0) { throw 'cargo build failed' }

& $compiler "/DAPP_VERSION=$version" (Join-Path $PSScriptRoot 'dn-killer.nsi')
if ($LASTEXITCODE -ne 0) { throw 'makensis failed' }
Get-Item -LiteralPath (Join-Path $root "target\DN-Killer-$version-x64-setup.exe")
