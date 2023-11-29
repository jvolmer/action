pub trait Observable {
    fn value(&self) -> i64;
}

#[derive(PartialEq, Debug)]
pub struct Correlation {
    value: i64,
}
impl Correlation {
    pub fn of(a: &impl Observable, b: &impl Observable) -> Self {
        Self {
            value: a.value() * b.value(),
        }
    }
    pub fn value(self) -> i64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct X {
        value: i64,
    }
    impl Observable for X {
        fn value(&self) -> i64 {
            self.value
        }
    }

    #[test]
    fn correlation_multiplies_values() {
        assert_eq!(
            Correlation::of(&X { value: 4 }, &X { value: 2 }),
            Correlation { value: 8 }
        );
        assert_eq!(
            Correlation::of(&X { value: -1 }, &X { value: 2 }),
            Correlation { value: -2 }
        );
        assert_eq!(
            Correlation::of(&X { value: -14 }, &X { value: -2 }),
            Correlation { value: 28 }
        );
        assert_eq!(
            Correlation::of(&X { value: 3 }, &X { value: -5 }),
            Correlation { value: -15 }
        );
    }
}
