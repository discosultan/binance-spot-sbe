# binance-spot-sbe

Binance Spot API SBE message schema.

## Development

### Downloading schemas

The latest SBE schemas are available from the [binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs/tree/master/sbe/schemas) repository.

```sh
curl -L -H "Accept: application/vnd.github.v3.raw" \
     -O https://api.github.com/repos/binance/binance-spot-api-docs/contents/sbe/schemas/spot_prod_latest.xml?ref=master

curl -L -H "Accept: application/vnd.github.v3.raw" \
     -O https://api.github.com/repos/binance/binance-spot-api-docs/contents/sbe/schemas/spot_fix_prod_latest.xml?ref=master
```

### Generating types

To generate types from the schemas, follow the steps in [binance-sbe-rust-sample-app](https://github.com/binance/binance-sbe-rust-sample-app?tab=readme-ov-file#updates).

```sh
./generate_spot.sh
./generate_spot_fix.sh
```
