## Build
### compile
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.13.0 ./
```

### generate schema
```
cargo schema
```

## Execute
### Store
```
fnsad tx wasm store ./artifacts/test.wasm --node=http://localhost:26658 --from=alice --keyring-backend=test --chain-id=simd-testing --gas=auto
```

### Instantiate
```
fnsad tx wasm instantiate 1 '{"sender":"link1zgn69sdcyg9wzypzlyrkqkmwn0g7qffn6kyfhh"}' --label=test --admin=link1zgn69sdcyg9wzypzlyrkqkmwn0g7qffn6kyfhh --node=http://localhost:26658 --chain-id=simd-testing --gas=auto --from=alice --keyring-backend=test
```

### Execute
**Send**
```
fnsad tx wasm execute link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8 '{"send":{"amount":"1","recipient":"link1zgn69sdcyg9wzypzlyrkqkmwn0g7qffn6kyfhh"}}' --from=alice --keyring-backend=test --node=http://localhost:26658 --chain-id=simd-testing -y
```
