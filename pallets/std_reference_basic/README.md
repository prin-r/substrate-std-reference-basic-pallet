# Std Reference Basic Pallet

### Overview

This pallet exposes capabilities for data relayer to relay external offchain data.
The relayers are assigned the role by the owner of the pallet. After the data is relayed to the storage, others pallet can call getter functions to view the data.

```sh
                        Owner 👑
                            |
                            |👇 Authorize an account to be a relayer.
                            |
                            V
         ___________________________________________ ... ___
        |               |              |                    |
    🕊 Relayer 1    🕊 Relayer 2    🕊 Releyer 3     ...  🕊 Relayer n
    |
    |                /‾‾‾
    |               /   [
    |              /       (<symbol 1>,<rate 1>,<resolve_time 1>,<request_id 1>),
    |             /        (<symbol 2>,<rate 2>,<resolve_time 2>,<request_id 2>),
    |relay data 📦         (<symbol 3>,<rate 3>,<resolve_time 3>,<request_id 3>),
    |             \         ...
    |              \       (<symbol n>,<rate n>,<resolve_time n>,<request_id n>),
    |               \   ]
    |                \___
    |
    V
    🏛 Storage
```

---

### Pallet

Pallets are a special kind of Rust module made up of a set of types, trait implementations and functions from which Substrate runtimes can be composed.

#### 🏛 Storages

Storage allows you to store data in your blockchain that is persisted between blocks and can be accessed from within your runtime logic.

- Owner
  - A single value of `AccountId` that represent an account of the owner.
- Relayers
  - A mapping(`AccountId` => `bool`)
  - key `AccountId`: Any `AccountId`.
  - value `bool` : A flag that indicates whether the `AccountId` has the authority to relay data.
- Refs
  - A mapping(`Vec<u8>` => (`u64`, `u64`, `u64`))
  - key `Vec<u8>` : A string symbol of the asset in bytes. For example "BTC" -> [66, 84, 67].
  - value (`u64`, `u64`, `u64`) : A tuple of (rate, resolve_time, request_id).
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | rate | `u64` |The asset's value in dollars|
    | resolve time | `u64` |A timestamp of the asset's value on Band chain|
    | request id | `u64` |A request id on Band chain that relevant to the asset's value and timestamp|

#### 🎉 Events

The pallet can emit events when it wants to notify external entities about changes or conditions in the runtime to external entities like users, chain explorers, or dApps.

- TransferOwnership(`AccountId`, `AccountId`)
  - This event will be emitted upon transfer of ownership.
  - Params
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | previous owner | `AccountId` |The previous owner|
    | new owner | `AccountId` |The new owner|
- SetRelayer(`AccountId`, `bool`)
  - This event will be emitted when an account is set as a relayer or cancels the relayer role of an account.
  - Params
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | relayer | `AccountId` |An account that is designated as a relayer or not as a relayer by the owner|
    | is_relayer | `bool` |A flag that indicates whether the above account is a relayer or not|
- RefDataUpdate(`Vec<u8>`, `u64`, `u64`, `u64`)
  - This event will be emitted every time data is written to the storage `Refs`. For this pallet, only `relay` function can change the data within `Refs`.
  - Params
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | symbol | `Vec<u8>` |A symbol of the asset|
    | rate | `u64` |The asset's value in dollars|
    | resolve time | `u64` |A timestamp of the asset's value on Band chain|
    | request id | `u64` |A request id on Band chain that relevant to the asset's value and timestamp|

#### 🚨 Errors

Runtime code should explicitly and gracefully handle all error cases, which is to say that runtime code must be "non-throwing", or must never "panic" to use Rust terminology. A common idiom for writing non-throwing Rust code is to write functions that return Result types. The Result enum type possesses an Err variant that allows a function to indicate that it failed to execute successfully without needing to panic.

- OwnerNotSet
  - This error will be raised when ownership has not yet been identified.
- NotAnOwner
  - This error will be raised when a non-owner account attempts to perform an action that only the owner can perform. For this pallet there are only 2 extrinsics that are intended for the owner: `transfer_ownership` and `set_relayer`.
- NotARelayer
  - This error will be raised when a non-relayer account attempts to relay anything into the pallet.

#### 🛸 Extrinsics

An extrinsic is a piece of information that comes from outside the chain and is included in a block. Extrinsics fall into three categories: inherents, signed transactions, and unsigned transactions.

- transfer_ownership(new_owner: `AccountId`)
  - Allows you to transfer ownership to another account. This function can only be used by the owner.
  - Params
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | new owner | `AccountId` |An account of the new owner|
- set_relayer(relayer: `AccountId`, is_relayer: `bool`)
  - Allows the owner to determine which account is a messenger or which account is not. This function can only be used by the owner.
  - Params
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | relayer | `AccountId` |An account that is designated as a relayer or not as a relayer by the owner|
    | is_relayer | `bool` |A flag that indicates whether the above account is a relayer or not|
- relay(values: `Vec<(Vec<u8>, u64, u64, u64)>`)
  - Allows the relayer to relay external data into the pallet. This function can only be used by the relayer.
  - Params
    1. values: A list of tuple that consists of 4 arguments:
       | Name | Type | Description|
       | ----------- | ----------- |------|
       | symbol | `Vec<u8>` |A symbol of the asset|
       | rate | `u64` |The asset's value in dollars|
       | resolve time | `u64` |A timestamp of the asset's value on Band chain|
       | request id | `u64` |A request id on Band chain that relevant to the asset's value and timestamp|

#### 🔮 Views

Functions that help other pallets to query information in this pallet.

- get_refs(symbol: `Vec<u8>`) -> `Option<(u64, u64, u64)>`
  - This function help query the raw data from the state for a given key.
  - Params
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | symbol | `Vec<u8>` |A symbol of the asset|
  - Output
    `Option` of `(u64, u64, u64)`
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | rate | `u64` |The asset's value in dollars|
    | resolve time | `u64` |A timestamp of the asset's value on Band chain|
    | request id | `u64` |A request id on Band chain that relevant to the asset's value and timestamp|
- get_ref_data(symbol: `Vec<u8>`) -> `Option<(u64, u64)>`
  - This function help query the `(value in dollars) × 1_000_000_000` and `timestamp` of a given symbol. If the symbol == "USD" then return `1 × 1_000_000_000` instead. If the given symbol is not available in storage `Refs` then return `None` instead.
  - Params
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | symbol | `Vec<u8>` |A symbol of the asset|
  - Output
    `Option` of `(u64, u64)`
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | rate | `u64` |The asset's value in dollars times 1_000_000_000|
    | resolve time | `u64` |A timestamp of the asset's value on Band chain|
- get_reference_data(base_symbol: `Vec<u8>`, quote_symbol: `Vec<u8>`) -> `Option<(u64, u64, u64)> `
  This function help query the relative value of 2 assets (base asset, quote asset) and the timestamps of both assets. If both of these assets are present in the storage `Refs`, return `(relative value × 1_000_000_000, timestamp of base asset, timestamp oof quote asset)`, but if not, return is `None`.
  - Params
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | base symbol | `Vec<u8>` |A symbol of the base asset|
    | quote symbol | `Vec<u8>` |A symbol of the quote asset|
  - Output
    `Option` of `(u64, u64, u64)`
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | relative rate | `u64` |relative value of both assets times 1_000_000_000|
    | base asset's timestamp | `u64` |A timestamp of the base asset's value on Band chain|
    | quote asset's timestamp | `u64` |A timestamp of the quote asset's value on Band chain|
- get_reference_data_bulk(base_quote_symbols: `Vec<(Vec<u8>, Vec<u8>)`>) -> `Option<Vec<(u64, u64, u64)>>`
  This function query list of relative value of 2 assets (list of asset pairs). This function is basically a batch of `get_reference_data`.
  - Params
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | list of pairs | `Vec<(Vec<u8>, Vec<u8>)` |A list of pairs of base symbol and quote symbol|
  - Output
    `Option` of `Vec<(u64, u64, u64)>`
    | Name | Type | Description|
    | ----------- | ----------- |------|
    | list of relative rate and timestamps | `u64` |A list that contain tuple of relative value of both assets times 1_000_000_000, base timestamp and quote timestamp|
