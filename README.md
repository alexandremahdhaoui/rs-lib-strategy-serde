# rs-lib-serde-strategy

## Disclaimer

- This library is intended at being a strategy injector to decouple `serde`, its implementations & usages.
- The library should allow plugging different serde implementations with different serde "users".
- This library is not intended at being an adapters, neither for implementations nor for "users".
- Both "users" & implementations are responsible for providing adapters or interfaces that comply to this bridge
specification.
- This library should not be a collector of interfaces to every possible implementations of the serde library.