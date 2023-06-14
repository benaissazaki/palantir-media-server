cd client
pnpm run build --emptyOutDir --outDir ..\build
cd ..
cargo build --release
copy target\release\palantir-media-server.exe build\
copy app_settings_example.json build\app_settings.json
