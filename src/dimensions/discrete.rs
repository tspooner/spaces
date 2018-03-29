use {Space, BoundedSpace, FiniteSpace, Surjection, Span};

use rand::{
    ThreadRng,
    distributions::{
        Range as RngRange,
        IndependentSample
    },
};
use serde::{
    Deserialize,
    Deserializer,
    de::{self, Visitor},
};
use std::{cmp, fmt, ops::Range};

/// A finite discrete dimension.
#[derive(Clone, Copy, Serialize)]
pub struct Discrete {
    size: usize,

    #[serde(skip_serializing)]
    ub: usize,

    #[serde(skip_serializing)]
    range: RngRange<usize>,
}

impl Discrete {
    pub fn new(size: usize) -> Discrete {
        Discrete {
            ub: size - 1,
            size: size,
            range: RngRange::new(0, size),
        }
    }
}

impl Space for Discrete {
    type Value = usize;

    fn dim(&self) -> usize { 1 }

    fn span(&self) -> Span { Span::Finite(self.size) }

    fn sample(&self, rng: &mut ThreadRng) -> usize { self.range.ind_sample(rng) }
}

impl BoundedSpace for Discrete {
    type BoundValue = usize;

    fn lb(&self) -> &usize { &0 }

    fn ub(&self) -> &usize { &self.ub }

    fn contains(&self, val: Self::Value) -> bool { val < self.size }
}

impl FiniteSpace for Discrete {
    fn range(&self) -> Range<Self::Value> { 0..self.size }
}

impl Surjection<usize, usize> for Discrete {
    fn map(&self, val: usize) -> usize { val as usize }
}

impl<'de> Deserialize<'de> for Discrete {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        enum Field {
            Size,
        };
        const FIELDS: &'static [&'static str] = &["size"];

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where D: Deserializer<'de>
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`size`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where E: de::Error
                    {
                        match value {
                            "size" => Ok(Field::Size),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DiscreteVisitor;

        impl<'de> Visitor<'de> for DiscreteVisitor {
            type Value = Discrete;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Discrete")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Discrete, V::Error>
                where V: de::SeqAccess<'de>
            {
                let size = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(Discrete::new(size))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Discrete, V::Error>
                where V: de::MapAccess<'de>
            {
                let mut size = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Size => {
                            if size.is_some() {
                                return Err(de::Error::duplicate_field("size"));
                            }

                            size = Some(map.next_value()?);
                        }
                    }
                }

                Ok(Discrete::new(size.ok_or_else(|| de::Error::missing_field("size"))?))
            }
        }

        deserializer.deserialize_struct("Discrete", FIELDS, DiscreteVisitor)
    }
}

impl cmp::PartialEq for Discrete {
    fn eq(&self, other: &Discrete) -> bool {
        self.size.eq(&other.size)
    }
}

impl fmt::Debug for Discrete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Discrete")
            .field("size", &self.size)
            .finish()
    }
}


#[cfg(test)]
mod tests {
    extern crate serde_test;

    use rand::thread_rng;
    use self::serde_test::{assert_tokens, Token};
    use super::*;

    #[test]
    fn test_span() {
        fn check(size: usize) {
            let d = Discrete::new(size);

            assert_eq!(d.span(), Span::Finite(size));
        }

        check(5);
        check(10);
        check(100);
    }

    #[test]
    fn test_sampling() {
        fn check(size: usize) {
            let d = Discrete::new(size);
            let mut rng = thread_rng();

            for _ in 0..100 {
                let s = d.sample(&mut rng);

                assert!(s < size);
            }
        }

        check(5);
        check(10);
        check(100);
    }

    #[test]
    fn test_bounds() {
        fn check(size: usize) {
            let d = Discrete::new(size);

            assert_eq!(d.lb(), &0);
            assert_eq!(d.ub(), &(size - 1));

            assert!(d.contains(0));
            assert!(d.contains((size - 1)));
            assert!(!d.contains(size));
        }

        check(5);
        check(10);
        check(100);
    }

    #[test]
    fn test_range() {
        assert_eq!(Discrete::new(1).range(), 0..1);
        assert_eq!(Discrete::new(5).range(), 0..5);
        assert_eq!(Discrete::new(10).range(), 0..10);
    }

    #[test]
    fn test_surjection() {
        let d = Discrete::new(10);

        assert_eq!(d.map(0), 0);
        assert_eq!(d.map(1), 1);
        assert_eq!(d.map(2), 2);
        assert_eq!(d.map(3), 3);
        assert_eq!(d.map(4), 4);
        assert_eq!(d.map(5), 5);
        assert_eq!(d.map(6), 6);
        assert_eq!(d.map(7), 7);
        assert_eq!(d.map(8), 8);
        assert_eq!(d.map(9), 9);
    }

    #[test]
    fn test_serialisation() {
        fn check(size: usize) {
            let d = Discrete::new(size);

            assert_tokens(&d,
                          &[Token::Struct {
                                name: "Discrete",
                                len: 1,
                            },
                            Token::Str("size"),
                            Token::U64(size as u64),
                            Token::StructEnd]);
        }

        check(5);
        check(10);
        check(100);
    }
}
