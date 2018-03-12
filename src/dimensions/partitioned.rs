use rand::distributions::{Range as RngRange, IndependentSample};
use serde::{Deserialize, Deserializer, de};
use serde::de::Visitor;
use std::{cmp, fmt};
use super::*;


/// A finite, uniformly partitioned continous dimension.
#[derive(Clone, Copy, Serialize)]
pub struct Partitioned {
    lb: f64,
    ub: f64,
    density: usize,

    #[serde(skip_serializing)]
    range: RngRange<f64>,
}

impl Partitioned {
    pub fn new(lb: f64, ub: f64, density: usize) -> Partitioned {
        Partitioned {
            lb: lb,
            ub: ub,
            density: density,

            range: RngRange::new(lb, ub),
        }
    }

    pub fn from_continuous(d: Continuous, density: usize) -> Partitioned {
        Partitioned {
            lb: d.lb,
            ub: d.ub,
            density: density,

            range: d.range,
        }
    }

    pub fn to_partition(&self, val: f64) -> usize {
        let clipped = clip!(self.lb, val, self.ub);

        let diff = clipped - self.lb;
        let range = self.ub - self.lb;

        let i = ((self.density as f64) * diff / range).floor() as usize;

        if i == self.density { i - 1 } else { i }
    }

    pub fn centres(&self) -> Vec<f64> {
        let w = (self.ub - self.lb) / self.density as f64;
        let hw = w / 2.0;

        (0..self.density).map(|i| self.lb + w * (i as f64) - hw).collect()
    }

    pub fn partition_width(&self) -> f64 {
        (self.lb - self.ub) / self.density as f64
    }

    pub fn density(&self) -> usize {
        self.density
    }
}

impl Dimension for Partitioned {
    type Value = usize;

    fn convert(&self, val: f64) -> Self::Value {
        self.to_partition(val)
    }

    fn span(&self) -> Span {
        Span::Finite(self.density)
    }

    fn sample(&self, rng: &mut ThreadRng) -> usize {
        self.to_partition(self.range.ind_sample(rng))
    }
}

impl BoundedDimension for Partitioned {
    type ValueBound = f64;

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

impl FiniteDimension for Partitioned {
    fn range(&self) -> Range<Self::Value> {
        0..(self.density + 1)
    }
}

impl<'de> Deserialize<'de> for Partitioned {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        enum Field {
            Lb,
            Ub,
            Density,
        };
        const FIELDS: &'static [&'static str] = &["lb", "ub", "density"];

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where D: Deserializer<'de>
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`lb`, `ub` or `density`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where E: de::Error
                    {
                        match value {
                            "lb" => Ok(Field::Lb),
                            "ub" => Ok(Field::Ub),
                            "density" => Ok(Field::Density),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct PartitionedVisitor;

        impl<'de> Visitor<'de> for PartitionedVisitor {
            type Value = Partitioned;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Partitioned")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Partitioned, V::Error>
                where V: de::SeqAccess<'de>
            {
                let lb = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let ub = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let density = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(Partitioned::new(lb, ub, density))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Partitioned, V::Error>
                where V: de::MapAccess<'de>
            {
                let mut lb = None;
                let mut ub = None;
                let mut density = None;

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
                        Field::Density => {
                            if density.is_some() {
                                return Err(de::Error::duplicate_field("density"));
                            }

                            density = Some(map.next_value()?);
                        }
                    }
                }

                let lb = lb.ok_or_else(|| de::Error::missing_field("lb"))?;
                let ub = ub.ok_or_else(|| de::Error::missing_field("ub"))?;
                let density = density.ok_or_else(|| de::Error::missing_field("density"))?;

                Ok(Partitioned::new(lb, ub, density))
            }
        }

        deserializer.deserialize_struct("Partitioned", FIELDS, PartitionedVisitor)
    }
}

impl cmp::PartialEq for Partitioned {
    fn eq(&self, other: &Partitioned) -> bool {
        self.lb.eq(&other.lb) && self.ub.eq(&other.ub) && self.density.eq(&other.density)
    }
}

impl fmt::Debug for Partitioned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Partitioned")
            .field("lb", &self.lb)
            .field("ub", &self.ub)
            .field("density", &self.density)
            .finish()
    }
}


#[cfg(test)]
mod tests {
    use rand::thread_rng;
    use serde_test::{assert_tokens, Token};
    use super::*;

    #[test]
    fn test_partitioned() {
        for (lb, ub, density) in vec![(0.0, 5.0, 5), (-5.0, 5.0, 10), (-5.0, 0.0, 5)] {
            let d = Partitioned::new(lb, ub, density);
            let mut rng = thread_rng();

            assert_eq!(d.span(), Span::Finite(density));

            assert!(!d.contains(ub));
            assert!(d.contains(lb));
            assert!(d.contains(((lb + ub) / 2.0)));

            for _ in 0..100 {
                let s = d.sample(&mut rng);
                assert!(s < density);
            }

            assert_tokens(&d,
                          &[Token::Struct {
                                name: "Partitioned",
                                len: 3,
                            },
                            Token::Str("lb"),
                            Token::F64(lb),
                            Token::Str("ub"),
                            Token::F64(ub),
                            Token::Str("density"),
                            Token::U64(density as u64),
                            Token::StructEnd]);
        }
    }
}
