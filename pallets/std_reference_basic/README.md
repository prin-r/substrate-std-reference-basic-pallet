# Std Reference Basic Pallet

### Overview

This pallet exposes capabilities for data relayer to relay external offchain data.
The relayers are assigned the role by the owner of the pallet.

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
