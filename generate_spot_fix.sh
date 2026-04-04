# Start from clean slate.
rm -rf binance-spot-fix-sbe

# Generate types from schema.
java \
    -Dsbe.output.dir=. \
    -Dsbe.target.language=Rust \
    -jar ../simple-binary-encoding/sbe-all/build/libs/sbe-all-1.38.0-SNAPSHOT.jar \
    spot_prod_latest.xml

# Rename package.
mv spot_sbe binance-spot-fix-sbe
cat > binance-spot-fix-sbe/Cargo.toml << 'EOF'
[package]
name = "binance-spot-fix-sbe"
version = "0.1.0"
authors = ["sbetool"]
description = "Binance Spot API FIX SBE message schema"
edition = "2024"
keywords = ["binance", "sbe", "spot", "fix"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/discosultan/binance-spot-sbe"
EOF

# Format and lint.
cargo fmt --package binance-spot-fix-sbe
cargo clippy --package binance-spot-fix-sbe
