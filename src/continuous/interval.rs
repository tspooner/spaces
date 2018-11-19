use core::{BoundedSpace, Space, Card, Surjection};
use serde::{Deserialize, Deserializer, de::{self, Visitor}};
use std::{
    cmp,
    fmt,
    f64::{INFINITY, NEG_INFINITY},
};

/// Type representing an interval on the real line.
#[derive(Clone, Copy, Serialize)]
pub struct Interval {
    pub(crate) lb: Option<f64>,
    pub(crate) ub: Option<f64>,
}

impl Interval {
    fn new(lb: Option<f64>, ub: Option<f64>) -> Interval {
        Interval {
            lb, ub,
        }
    }

    pub fn bounded(lb: f64, ub: f64) -> Interval {
        Interval::new(Some(lb), Some(ub))
    }

    pub fn left_bounded(lb: f64) -> Interval {
        Interval::new(Some(lb), None)
    }

    pub fn right_bounded(ub: f64) -> Interval {
        Interval::new(None, Some(ub))
    }
}

impl Space for Interval {
    type Value = f64;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Infinite }
}

impl BoundedSpace for Interval {
    type BoundValue = Self::Value;

    fn inf(&self) -> Option<f64> { self.lb }

    fn sup(&self) -> Option<f64> { self.ub }

    fn contains(&self, val: Self::BoundValue) -> bool {
        self.lb.map_or(true, |inf| val >= inf) && self.ub.map_or(true, |sup| val <= sup)
    }
}

impl Surjection<f64, f64> for Interval {
    fn map(&self, val: f64) -> f64 {
        let val = self.lb.map_or(val, |inf| val.max(inf));
        let val = self.ub.map_or(val, |sup| val.min(sup));

        val
    }
}

impl<'de> Deserialize<'de> for Interval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        enum Field {
            Lb,
            Ub,
        };
        const FIELDS: &'static [&'static str] = &["lb", "ub"];

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where D: Deserializer<'de> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`lb` or `ub`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where E: de::Error {
                        match value {
                            "lb" => Ok(Field::Lb),
                            "ub" => Ok(Field::Ub),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct IntervalVisitor;

        impl<'de> Visitor<'de> for IntervalVisitor {
            type Value = Interval;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Interval")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Interval, V::Error>
            where V: de::SeqAccess<'de> {
                let lb = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let ub = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Interval::new(lb, ub))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Interval, V::Error>
            where V: de::MapAccess<'de> {
                let mut lb = None;
                let mut ub = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Lb => {
                            if lb.is_some() {
                                return Err(de::Error::duplicate_field("lb"));
                            }

                            lb = Some(map.next_value()?);
                        },
                        Field::Ub => {
                            if ub.is_some() {
                                return Err(de::Error::duplicate_field("ub"));
                            }

                            ub = Some(map.next_value()?);
                        },
                    }
                }

                let lb = lb.ok_or_else(|| de::Error::missing_field("lb"))?;
                let ub = ub.ok_or_else(|| de::Error::missing_field("ub"))?;

                Ok(Interval::new(lb, ub))
            }
        }

        deserializer.deserialize_struct("Interval", FIELDS, IntervalVisitor)
    }
}

impl cmp::PartialEq for Interval {
    fn eq(&self, other: &Interval) -> bool { self.lb.eq(&other.lb) && self.ub.eq(&other.ub) }
}

impl fmt::Debug for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Interval")
            .field("lb", &self.lb)
            .field("ub", &self.ub)
            .finish()
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.lb.unwrap_or(NEG_INFINITY), self.ub.unwrap_or(INFINITY))
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_test;

    use self::serde_test::{assert_tokens, Token};
    use super::*;

    #[test]
    fn test_card() {
        fn check(lb: f64, ub: f64) {
            let d = Interval::bounded(lb, ub);

            assert_eq!(d.card(), Card::Infinite);
        }

        check(0.0, 5.0);
        check(-5.0, 5.0);
        check(-5.0, 0.0);
    }

    #[test]
    fn test_bounds() {
        fn check(lb: f64, ub: f64) {
            let d = Interval::bounded(lb, ub);

            assert_eq!(d.inf().unwrap(), lb);
            assert_eq!(d.sup().unwrap(), ub);

            assert!(d.contains(ub));
            assert!(d.contains(lb));
            assert!(d.contains((lb + ub) / 2.0));
        }

        check(0.0, 5.0);
        check(-5.0, 5.0);
        check(-5.0, 0.0);
    }

    #[test]
    fn test_surjection() {
        let d = Interval::bounded(0.0, 5.0);

        assert_eq!(d.map(-5.0), 0.0);
        assert_eq!(d.map(0.0), 0.0);
        assert_eq!(d.map(2.5), 2.5);
        assert_eq!(d.map(5.0), 5.0);
        assert_eq!(d.map(10.0), 5.0);
    }

    #[test]
    fn test_serialisation() {
        fn check(lb: f64, ub: f64) {
            let d = Interval::bounded(lb, ub);

            assert_tokens(
                &d,
                &[
                    Token::Struct {
                        name: "Interval",
                        len: 2,
                    },
                    Token::Str("lb"),
                    Token::Some,
                    Token::F64(lb),
                    Token::Str("ub"),
                    Token::Some,
                    Token::F64(ub),
                    Token::StructEnd,
                ],
            );
        }

        check(0.0, 5.0);
        check(-5.0, 5.0);
        check(-5.0, 0.0);
    }
}
