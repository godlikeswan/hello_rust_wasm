$wasm = [System.IO.File]::ReadAllBytes("target/wasm32-unknown-unknown/release/hello_rust_wasm.wasm")
$wasmBase64 = [Convert]::ToBase64String($wasm)
$template = Get-Content -Raw "template.html"
$html = $template.Replace("{{WASM}}", $wasmBase64)
New-Item -ItemType Directory -Force -Path "dist" | Out-Null
$html | Out-File -FilePath "dist/index.html" -Encoding utf8
