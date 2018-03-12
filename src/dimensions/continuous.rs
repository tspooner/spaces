use rand::distributions::{Range as RngRange, IndependentSample};
use serde::{Deserialize, Deserializer, de};
use serde::de::Visitor;
use super::*;
use std::{cmp, fmt};

/// A continous dimension.
#[derive(Clone, Copy, Serialize)]
pub struct Continuous {
    pub(super) lb: f64,
    pub(super) ub: f64,

    #[serde(skip_serializing)]
    pub(super) range: RngRange<f64>,
}

impl Continuous {
    pub fn new(lb: f64, ub: f64) -> Continuous {
        Continuous {
            lb: lb,
            ub: ub,
            range: RngRange::new(lb, ub),
        }
    }
}

impl Dimension for Continuous {
    type Value = f64;

    fn convert(&self, val: f64) -> Self::Value {
        clip!(self.lb, val, self.ub)
    }

    fn span(&self) -> Span {
        Span::Infinite
    }

    fn sample(&self, rng: &mut ThreadRng) -> f64 {
        self.range.ind_sample(rng)
    }
}

impl BoundedDimension for Continuous {
    type ValueBound = Self::Value;

    fn lb(&self) -> &f64 {
        &self.lb
    }

    fn ub(&self) -> &f64 {
        &self.ub
    }

    fn contains(&self, val: Self::ValueBound) -> bool {
        (val >= self.lb) && (val < self.ub)
    }

    fn is_infinite(&self) -> bool {
        self.lb.is_infinite() || self.ub.is_infinite()
    }
}

impl<'de> Deserialize<'de> for Continuous {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        enum Field {
            Lb,
            Ub,
        };
        const FIELDS: &'static [&'static str] = &["lb", "ub"];

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where D: Deserializer<'de>
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`lb` or `ub`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where E: de::Error
                    {
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

        struct ContinuousVisitor;

        impl<'de> Visitor<'de> for ContinuousVisitor {
            type Value = Continuous;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Continuous")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Continuous, V::Error>
                where V: de::SeqAccess<'de>
            {
                let lb = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let ub = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                Ok(Continuous::new(lb, ub))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Continuous, V::Error>
                where V: de::MapAccess<'de>
            {
                let mut lb = None;
                let mut ub = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Lb => {
                            if lb.is_some() {
                                return Err(de::Error::duplicate_field("lb"));
                            }

                            lb = Some(map.next_value()?);
                        }
                        Field::Ub => {
                            if ub.is_some() {
                                return Err(de::Error::duplicate_field("ub"));
                            }

                            ub = Some(map.next_value()?);
                        }
                    }
                }

                let lb = lb.ok_or_else(|| de::Error::missing_field("lb"))?;
                let ub = ub.ok_or_else(|| de::Error::missing_field("ub"))?;

                Ok(Continuous::new(lb, ub))
            }
        }

        deserializer.deserialize_struct("Continuous", FIELDS, ContinuousVisitor)
    }
}

impl cmp::PartialEq for Continuous {
    fn eq(&self, other: &Continuous) -> bool {
        self.lb.eq(&other.lb) && self.ub.eq(&other.ub)
    }
}

impl fmt::Debug for Continuous {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Continuous")
            .field("lb", &self.lb)
            .field("ub", &self.ub)
            .finish()
    }
}


#[cfg(test)]
mod tests {
    use rand::thread_rng;
    use serde_test::{assert_tokens, Token};
    use super::*;

    #[test]
    fn test_continuous() {
        for (lb, ub) in vec![(0.0, 5.0), (-5.0, 5.0), (-5.0, 0.0)] {
            let d = Continuous::new(lb, ub);
            let mut rng = thread_rng();

            assert_eq!(d.span(), Span::Infinite);

            assert!(!d.contains(ub));
            assert!(d.contains(lb));
            assert!(d.contains(((lb + ub) / 2.0)));

            for _ in 0..100 {
                let s = d.sample(&mut rng);
                assert!(s < ub);
                assert!(s >= lb);
                assert!(d.contains(s));
            }

            assert_tokens(&d,
                          &[Token::Struct {
                                name: "Continuous",
                                len: 2,
                            },
                            Token::Str("lb"),
                            Token::F64(lb),
                            Token::Str("ub"),
                            Token::F64(ub),
                            Token::StructEnd]);
        }
    }
}
