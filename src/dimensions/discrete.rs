use rand::distributions::{Range as RngRange, IndependentSample};
use serde::{Deserialize, Deserializer, de};
use serde::de::Visitor;
use std::{cmp, fmt};
use super::*;

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

impl Dimension for Discrete {
    type Value = usize;

    fn sample(&self, rng: &mut ThreadRng) -> usize {
        self.range.ind_sample(rng)
    }

    fn convert(&self, val: f64) -> Self::Value {
        val as usize
    }

    fn span(&self) -> Span {
        Span::Finite(self.size)
    }
}

impl BoundedDimension for Discrete {
    type ValueBound = usize;

    fn lb(&self) -> &usize {
        &0
    }

    fn ub(&self) -> &usize {
        &self.ub
    }

    fn contains(&self, val: Self::Value) -> bool {
        val < self.size
    }

    fn is_infinite(&self) -> bool {
        false
    }
}

impl FiniteDimension for Discrete {
    fn range(&self) -> Range<Self::Value> {
        0..self.size
    }
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
    use super::*;

    #[test]
    fn test_discrete() {
        for size in vec![5, 10, 100] {
            let d = Discrete::new(size);
            let mut rng = thread_rng();

            assert_eq!(d.span(), Span::Finite(size));

            assert!(!d.contains(size));

            assert!(d.contains(0));
            assert!(d.contains((size - 1)));

            for _ in 0..100 {
                let s = d.sample(&mut rng);
                assert!(s < size);
            }

            assert_tokens(&d,
                          &[Token::Struct {
                                name: "Discrete",
                                len: 1,
                            },
                            Token::Str("size"),
                            Token::U64(size as u64),
                            Token::StructEnd]);
        }
    }
}
