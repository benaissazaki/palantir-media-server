cd client
pnpm install
pnpm run build --emptyOutDir --outDir ../build
cd ..
cargo build --release
cp target/release/palantir-media-server build/
cp app_settings_example.json build/app_settings.json
