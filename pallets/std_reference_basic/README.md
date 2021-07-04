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

#### Storages

Storage allows you to store data in your blockchain that is persisted between blocks and can be accessed from within your runtime logic.

- Owner
  - A single value of `AccountId`
- Relayers
  - mapping(`AccountId` => `bool`)
- Refs
  - mapping(`Vec<u8>` => (`u64`, `u64`, `u64`))

#### Events

The pallet can emit events when it wants to notify external entities about changes or conditions in the runtime to external entities like users, chain explorers, or dApps.

- TransferOwnership(`AccountId`, `AccountId`)
  - This event will be emitted upon transfer of ownership.
  - Params
    1. `AccountId` is the previous owner.
    2. `AccountId` is the new owner.
- SetRelayer(`AccountId`, `bool`)
  - This event will be emitted when an account is set as a relayer or cancels the relayer role of an account.
  - Params
    1. `AccountId` is an account that is designated as a relayer or not as a relayer.
    2. `bool` is a flag that indicates whether the above account is a relayer or not.
- RefDataUpdate(`Vec<u8>`, `u64`, `u64`, `u64`)
  - This event will be emitted every time data is written to the storage `Refs`. For this pallet, only `relay` function can change the data within `Refs`.

#### Errors

Runtime code should explicitly and gracefully handle all error cases, which is to say that runtime code must be "non-throwing", or must never "panic" to use Rust terminology. A common idiom for writing non-throwing Rust code is to write functions that return Result types. The Result enum type possesses an Err variant that allows a function to indicate that it failed to execute successfully without needing to panic.

- OwnerNotSet
  - This error will be raised when ownership has not yet been identified.
- NotAnOwner
  - This error will be raised when a non-owner account attempts to perform an action that only the owner can perform. For this pallet there are only 2 extrinsics that are intended for the owner: `transfer_ownership` and `set_relayer`.
- NotARelayer
  - This error will be raised when a non-relayer account attempts to relay anything into the pallet.

#### Extrinsics

An extrinsic is a piece of information that comes from outside the chain and is included in a block. Extrinsics fall into three categories: inherents, signed transactions, and unsigned transactions.

- transfer_ownership(new_owner: `AccountId`)
  - Allows you to transfer ownership to another account. This function can only be used by the owner.ม To change the owner, call `transfer_ownership(address new_owner)` as the owner.
- set_relayer(relayer: `AccountId`, is_relayer: `bool`)
  - ...
- relay(values: `Vec<(Vec<u8>, u64, u64, u64)>`)
  - ...

#### Views

- get_refs(symbol: `Vec<u8>`) -> `Option<(u64, u64, u64)>`
- get_ref_data(symbol: `Vec<u8>`) -> `Option<(u64, u64)>`
- get_reference_data(base_symbol: `Vec<u8>`, quote_symbol: `Vec<u8>`) -> `Option<(u64, u64, u64)> `
- get_reference_data_bulk(base_quote_symbols: `Vec<(Vec<u8>, Vec<u8>)`>) -> `Option<Vec<(u64, u64, u64)>>`
