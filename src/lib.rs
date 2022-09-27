use serde::{Deserialize, Serialize};

trait Strategy {
    fn serialize<T>(&self, value: &T) -> Option<String> where T: ?Sized + Serialize;
    fn deserialize<'a, T>(&self, s: &'a str) -> Option<T> where T: Deserialize<'a>;
}


struct StrategyInjector<S: Strategy> {
    strategy: S
}

impl<S: Strategy> StrategyInjector<S> {
    pub fn new(strategy: S) -> StrategyInjector<S> {
        Self {strategy}
    }

    pub fn serialize<T: Serialize>(&self, value: T) -> Option<String> {
        self.strategy.serialize(&value)
    }

    pub fn deserialize<'a, R: Deserialize<'a>>(&self, s: &'a str) -> Option<R> {
        self.strategy.deserialize(s)
    }
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde::{Serialize, Deserialize};
    use serde_json;
    use crate::{StrategyInjector, Strategy};

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Test {string: String}

    fn obj() -> Test {
       Test{ string: "test".to_string() }
    }

    fn string() -> &'static str {
        r#"{"string":"test"}"#
    }

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

    #[test]
    fn json_de() {
        let i = string();
        let e = obj();
        let o: Test = serde_json::from_str(&i).unwrap();
        assert_eq!(e, o)
    }

    #[test]
    fn json_ser() {
        let i = obj();
        let e = string();
        let o = serde_json::to_string(&i).unwrap();
        assert_eq!(e, o)
    }

    #[test]
    fn strategy_de() {
        let i = string();
        let e = obj();
        let o: Test = JsonStrategy.deserialize::<Test>(i).unwrap();
        assert_eq!(e, o)
    }

    #[test]
    fn strategy_ser() {
        let i = obj();
        let e = string();
        let o = JsonStrategy.serialize(&i).unwrap();
        assert_eq!(e, o )
    }

    #[test]
    fn serde_de() {
        let i = string();
        let e = obj();

        let ser_de = StrategyInjector::new(JsonStrategy);
        let o: Test = ser_de.deserialize::<Test>(i).unwrap();
        assert_eq!(e, o)
    }

    #[test]
    fn serde_ser() {
        let i = obj();
        let e = string();

        let ser_de = StrategyInjector::new(JsonStrategy);
        let o = ser_de.serialize(&i).unwrap();
        assert_eq!(e, o )
    }
}