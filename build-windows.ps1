# Change directory to 'client'
Set-Location -Path .\client

# Run 'pnpm install'
pnpm install --reporter silent

# Run 'pnpm run build'
pnpm run build --emptyOutDir --outDir ..\build

# Change directory back to the root
Set-Location -Path ..

# Run 'cargo build --release'
cargo build --release

# Copy files
Copy-Item -Path .\target\release\palantir-media-server.exe -Destination .\build\
Copy-Item -Path .\app_settings_example.json -Destination .\build\app_settings.json
