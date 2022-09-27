# rs-lib-strategy-serde

## Usage

The following code snippet shows how to create your own `serde_json` Strategy &
how easily you can use the `StrategyInjector`.
```rust
struct JsonStrategy;
impl Strategy for JsonStrategy {
    fn serialize<T>(&self, value: &T) -> Option<String> where T: ?Sized + Serialize {
        match serde_json::to_string(value) {
            Ok(res) => Some(res),
            _ => None
        }
    }

    fn deserialize<'a, T>(&self, s: &'a str) -> Option<T> where T: Deserialize<'a> {
        match serde_json::from_str::<T>(s) {
            Ok(res) => Some(res),
            _ => None
        }
    }
}
let ser_de = StrategyInjector::new(JsonStrategy);
ser_de.deserialize::<YourStruct>(input)
```

## Disclaimer

- This library is intended at being a strategy injector to decouple `serde`, its implementations & usages.
- The library should allow plugging different serde implementations with different serde "users".
- This library is not intended at being an adapters, neither for implementations nor for "users".
- Both "users" & implementations are responsible for providing adapters or interfaces that comply to this strategy
specification.
- This library should not be a collector of interfaces to every possible implementations of the serde library.