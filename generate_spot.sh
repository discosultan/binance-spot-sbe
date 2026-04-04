# Start from clean slate.
rm -rf binance-spot-sbe

# Generate types from schema.
java \
    -Dsbe.output.dir=. \
    -Dsbe.target.language=Rust \
    -jar ../simple-binary-encoding/sbe-all/build/libs/sbe-all-1.38.0-SNAPSHOT.jar \
    spot_prod_latest.xml

# Rename package.
mv spot_sbe binance-spot-sbe
cat > binance-spot-sbe/Cargo.toml << 'EOF'
[package]
name = "binance-spot-sbe"
version = "0.1.0"
authors = ["sbetool"]
description = "Binance Spot API SBE message schema"
edition = "2024"
keywords = ["binance", "sbe", "spot"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/discosultan/binance-spot-sbe"
EOF

# Format and lint.
cargo fmt --package binance-spot-sbe
cargo clippy --package binance-spot-sbe
