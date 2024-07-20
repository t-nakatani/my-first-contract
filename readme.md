# Injective 入門 (Rust)

## 概要
* Injectiveが用意しているガイドを見てcluade利用で数をカウントするコントラクトを作成
* Rustの書き方や、コントラクトの概念が集約されているのは、[contract.rs](/src/contract.rs)
* EVMにおけるABIみたいなものは、[schema](/schema)
* CODE_IDというのを求められるが、コントラクト作成時の[event logs](https://testnet.explorer.injective.network/transaction/89A43BE7B0EDB0318C4FBDF15AA7645C7C18E82859DE45B00E1C5DFF7754B985/event-logs/)に載っている

## link
* https://docs.injective.network/develop/guides/injective-101/testnet-deployment-guide
* https://claude.ai/chat/0018c49d-e21e-4760-9a03-8a55114c7740
* https://testnet.explorer.injective.network/account/inj17p6y38uhq0z497gux0ewz0skcf2eggzyf5dtff/transactions/


```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer-arm64:0.12.12
```

```
docker run --name="injective-core-staging" -v $(pwd)/artifacts:/var/artifacts --platform linux/amd64 --entrypoint=sh public.ecr.aws/l9h3g6c6/injective-core:staging -c "tail -f /dev/null"
```

```sh
docker exec -it injective-core-staging sh
```

```
/apps/data # injectived keys add testuser
Enter keyring passphrase:
Re-enter keyring passphrase:

- name: testuser
  type: local
  address: inj17p6y38uhq0z497gux0ewz0skcf2eggzyf5dtff
  pubkey: '{"@type":"/injective.crypto.v1beta1.ethsecp256k1.PubKey","key":"A/dkQ3xi6P5t6I6QA7Td7cLVfVXB1SvJKYcYFlsZuu09"}'
  mnemonic: ""
```


```sh
/apps/data # 
yes 12345678 | injectived tx wasm store /var/artifacts/my_first_contract-aarch64.wasm --from=inj17p6y38uhq0z497gux0ewz0skcf2eggzyf5dtff --chain-id="injective-888" --yes --gas-prices=160000000inj --gas=20000000 --node=https://testnet.sentry.tm.injective.network:443
code: 0
codespace: ""
data: ""
events: []
gas_used: "0"
gas_wanted: "0"
height: "0"
info: ""
logs: []
raw_log: ""
timestamp: ""
tx: null
txhash: 89A43BE7B0EDB0318C4FBDF15AA7645C7C18E82859DE45B00E1C5DFF7754B985
```

```
injectived query tx 89A43BE7B0EDB0318C4FBDF15AA7645C7C18E82859DE45B00E1C5DFF7754B985 --node=https://testnet.sentry.tm.injective.network:443
````


```
INIT='{"count":99}'
yes 12345678 | injectived tx wasm instantiate "$CODE_ID" "$INIT" --label="CounterTestInstance" --from=$(echo $INJ_ADDRESS) --chain-id="injective-888" --yes --gas-prices=160000000inj --gas=20000000 --no-admin --node=https://testnet.sentry.tm.injective.network:443
```

```
GET_COUNT_QUERY='{"get_count":{}}'
NODE="https://testnet.sentry.tm.injective.network:443"
CONTRACT_ADDRESS=inj10fvy3re4f2ec6m08r6vrl5st4w48xm3j97c0lp

injectived query wasm contract-state smart inj10fvy3re4f2ec6m08r6vrl5st4w48xm3j97c0lp "$GET_COUNT_QUERY" --node="$NODE" --output json

# {"data":{"count":99}}
```

```sh
INCREMENT='{"increment":{}}'
yes 12345678 | injectived tx wasm execute "$CONTRACT_ADDRESS" "$INCREMENT" --from=$(echo $INJ_ADDRESS) --chain-id="injective-888" --yes --gas-prices=160000000inj --gas=20000000 --node="$NODE" --output json

/apps/data # injectived query wasm contract-state smart inj10fvy3re4f2ec6m08r6vrl5st4w48xm3j97c0lp "$GET_COUNT_QUERY" --node="$NODE" -
-output json
{"data":{"count":100}}
```

```sh
RESET='{"reset":{"count":999}}'
yes 12345678 | injectived tx wasm execute "$CONTRACT_ADDRESS" "$RESET" --from=$(echo $INJ_ADDRESS) --chain-id="injective-888"  --yes --gas-prices=160000000inj --gas=20000000 --node="$NODE" --output json
```

```sh
/apps/data # injectived query wasm contract-state smart inj10fvy3re4f2ec6m08r6vrl5st4w48xm3j97c0lp "$GET_COUNT_QUERY" --node="$NODE" --output
 json
{"data":{"count":999}}
```
