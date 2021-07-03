# Std Reference Basic Pallet

### Overview

This pallet exposes capabilities for data relayer to relay external offchain data.
The relayers are assigned the role by the owner of the pallet. After the data is relayed to the storage, others pallet can call getter functions to view the data.

```sh
                               Owner 👑
                                  |
         ___________________________________________ ... ___
        |               |              |                    |
    🕊 Relayer 1    🕊 Relayer 2    🕊 Releyer 3     ...  🕊 Relayer n
    |
    | 📦 [
    |     (<symbol 1>,<rate 1>,<resolve_time 1>,<request_id 1>),
    |     (<symbol 2>,<rate 2>,<resolve_time 2>,<request_id 2>),
    |     (<symbol 3>,<rate 3>,<resolve_time 3>,<request_id 3>),
    |      ...
    |     (<symbol n>,<rate n>,<resolve_time n>,<request_id n>),
    |    ]
    |
    v
    🏛 Storage
```

### Pallet

#### Storages

- Owner
  - A single value of `AccountId`
- Relayers
  - mapping(`AccountId` => `bool`)
- Refs
  - mapping(`Vec<u8>` => (`u64`, `u64`, `u64`))

#### Events

- TransferOwnership(`AccountId`, `AccountId`)
- SetRelayer(`AccountId`, `bool`)
- RefDataUpdate(`Vec<u8>`, `u64`, `u64`, `u64`)

#### Errors

- OwnerNotSet
- NotAnOwner
- RelayerNotSet
- NotARelayer

#### Extrinsics

- transfer_ownership
- set_relayer
- relay

#### Calls

- get_refs(symbol: `Vec<u8>`) -> `Option<(u64, u64, u64)>`
- get_ref_data(symbol: `Vec<u8>`) -> `Option<(u64, u64)>`
- get_reference_data(base_symbol: `Vec<u8>`, quote_symbol: `Vec<u8>`) -> `Option<(u64, u64, u64)> `
- get_reference_data_bulk(base_quote_symbols: `Vec<(Vec<u8>, Vec<u8>)`>) -> `Option<Vec<(u64, u64, u64)>>`
