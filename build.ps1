$OutDir = "bin"

if (!(Test-Path $OutDir))
{
  mkdir $OutDir
}

# Empty OutDir
Get-ChildItem $OutDir | Remove-Item -Recurse -Force

# Write build.json
@{
  version = (git describe --abbrev=0 --tags)
  gitHash = (git rev-parse HEAD)
  buildTime = (Get-Date -Format "yyyy-MM-dd HH:mm:ss zzz")
} | ConvertTo-Json > app/build.json

# Build CLI
nimble install --depsOnly -y
nim c -d:release -d:strip --opt: size --outdir: bin cli/*.nim

# Build GUI
go install github.com/akavel/rsrc@latest
go install github.com/GizmoOAO/go-asset-bundle/cmd/goab-cli@latest
go generate
go build --ldflags "-s -w -H windowsgui" -o "$OutDir/paste2vrchat.exe"

# Run UPX
if ((Get-Command upx -ErrorAction SilentlyContinue) -ne $null)
{
  upx -9 "$OutDir/*.exe"
}
