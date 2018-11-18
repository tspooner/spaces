use continuous::Interval;
use core::{BoundedSpace, FiniteSpace, Space, Card, Surjection};
use rand::{Rng, distributions::{Distribution, Uniform}};
use serde::{Deserialize, Deserializer, de::{self, Visitor}};
use std::{cmp, fmt, ops::Range};

/// Type representing a finite, uniformly partitioned interval.
#[derive(Clone, Copy, Serialize)]
pub struct Partition {
    lb: f64,
    ub: f64,
    density: usize,

    #[serde(skip_serializing)]
    range: Uniform<f64>,
}

impl Partition {
    pub fn new(lb: f64, ub: f64, density: usize) -> Partition {
        Partition {
            lb: lb,
            ub: ub,
            density: density,

            range: Uniform::new_inclusive(lb, ub),
        }
    }

    pub fn from_continuous<I: Into<Interval>>(d: I, density: usize) -> Partition {
        let interval = d.into();

        Partition {
            lb: interval.lb.expect("Must be a bounded interval."),
            ub: interval.ub.expect("Must be a bounded interval."),
            density: density,

            range: interval.range.expect("Must be a bounded interval."),
        }
    }

    #[inline]
    pub fn density(&self) -> usize { self.density }

    #[inline]
    pub fn partition_width(&self) -> f64 { (self.ub - self.lb) / self.density as f64 }

    pub fn centres(&self) -> Vec<f64> {
        let w = self.partition_width();
        let hw = w / 2.0;

        (0..self.density)
            .map(|i| self.lb + w * ((i + 1) as f64) - hw)
            .collect()
    }

    pub fn to_partition(&self, val: f64) -> usize {
        let clipped = clip!(self.lb, val, self.ub);

        let diff = clipped - self.lb;
        let range = self.ub - self.lb;

        let i = ((self.density as f64) * diff / range).floor() as usize;

        if i == self.density {
            i - 1
        } else {
            i
        }
    }
}

impl Space for Partition {
    type Value = usize;

    fn dim(&self) -> usize { 1 }

    fn card(&self) -> Card { Card::Finite(self.density) }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        self.to_partition(self.range.sample(rng))
    }
}

impl BoundedSpace for Partition {
    type BoundValue = f64;

    fn inf(&self) -> Option<f64> { Some(self.lb) }

    fn sup(&self) -> Option<f64> { Some(self.ub) }

    fn contains(&self, val: Self::BoundValue) -> bool { (val >= self.lb) && (val < self.ub) }
}

impl FiniteSpace for Partition {
    fn range(&self) -> Range<Self::Value> { 0..self.density }
}

impl Surjection<f64, usize> for Partition {
    fn map(&self, val: f64) -> usize { self.to_partition(val) }
}

impl Surjection<usize, usize> for Partition {
    fn map(&self, val: usize) -> usize { clip!(0, val, self.density - 1) }
}

impl<'de> Deserialize<'de> for Partition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        enum Field {
            Lb,
            Ub,
            Density,
        };
        const FIELDS: &'static [&'static str] = &["lb", "ub", "density"];

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where D: Deserializer<'de> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`lb`, `ub` or `density`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where E: de::Error {
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

        struct PartitionVisitor;

        impl<'de> Visitor<'de> for PartitionVisitor {
            type Value = Partition;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Partition")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Partition, V::Error>
            where V: de::SeqAccess<'de> {
                let lb = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let ub = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let density = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(Partition::new(lb, ub, density))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Partition, V::Error>
            where V: de::MapAccess<'de> {
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
                        },
                        Field::Ub => {
                            if ub.is_some() {
                                return Err(de::Error::duplicate_field("ub"));
                            }

                            ub = Some(map.next_value()?);
                        },
                        Field::Density => {
                            if density.is_some() {
                                return Err(de::Error::duplicate_field("density"));
                            }

                            density = Some(map.next_value()?);
                        },
                    }
                }

                let lb = lb.ok_or_else(|| de::Error::missing_field("lb"))?;
                let ub = ub.ok_or_else(|| de::Error::missing_field("ub"))?;
                let density = density.ok_or_else(|| de::Error::missing_field("density"))?;

                Ok(Partition::new(lb, ub, density))
            }
        }

        deserializer.deserialize_struct("Partition", FIELDS, PartitionVisitor)
    }
}

impl cmp::PartialEq for Partition {
    fn eq(&self, other: &Partition) -> bool {
        self.lb.eq(&other.lb) && self.ub.eq(&other.ub) && self.density.eq(&other.density)
    }
}

impl fmt::Debug for Partition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Partition")
            .field("lb", &self.lb)
            .field("ub", &self.ub)
            .field("density", &self.density)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_test;

    use self::serde_test::{assert_tokens, Token};
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_from_continuous() {
        assert_eq!(
            Partition::new(0.0, 5.0, 5),
            Partition::from_continuous(Interval::bounded(0.0, 5.0), 5)
        );
    }

    #[test]
    fn test_density() {
        assert_eq!(Partition::new(0.0, 5.0, 5).density(), 5);
        assert_eq!(Partition::new(0.0, 5.0, 10).density(), 10);
        assert_eq!(Partition::new(-5.0, 5.0, 100).density(), 100);
    }

    #[test]
    fn test_partition_width() {
        assert_eq!(Partition::new(0.0, 5.0, 5).partition_width(), 1.0);
        assert_eq!(Partition::new(0.0, 5.0, 10).partition_width(), 0.5);
        assert_eq!(Partition::new(-5.0, 5.0, 10).partition_width(), 1.0);
    }

    #[test]
    fn test_centres() {
        assert_eq!(
            Partition::new(0.0, 5.0, 5).centres(),
            vec![0.5, 1.5, 2.5, 3.5, 4.5]
        );

        assert_eq!(
            Partition::new(-5.0, 5.0, 5).centres(),
            vec![-4.0, -2.0, 0.0, 2.0, 4.0]
        );
    }

    #[test]
    fn test_to_partition() {
        let d = Partition::new(0.0, 5.0, 6);

        assert_eq!(d.to_partition(-1.0), 0);
        assert_eq!(d.to_partition(0.0), 0);
        assert_eq!(d.to_partition(1.0), 1);
        assert_eq!(d.to_partition(2.0), 2);
        assert_eq!(d.to_partition(3.0), 3);
        assert_eq!(d.to_partition(4.0), 4);
        assert_eq!(d.to_partition(5.0), 5);
        assert_eq!(d.to_partition(6.0), 5);
    }

    #[test]
    fn test_dim() {
        assert_eq!(Partition::new(0.0, 5.0, 5).dim(), 1);
        assert_eq!(Partition::new(0.0, 5.0, 10).dim(), 1);
        assert_eq!(Partition::new(-5.0, 5.0, 10).dim(), 1);
    }

    #[test]
    fn test_card() {
        fn check(lb: f64, ub: f64, density: usize) {
            let d = Partition::new(lb, ub, density);

            assert_eq!(d.card(), Card::Finite(density));
        }

        check(0.0, 5.0, 5);
        check(-5.0, 5.0, 10);
        check(-5.0, 0.0, 5);
    }

    #[test]
    fn test_sampling() {
        fn check(lb: f64, ub: f64, density: usize) {
            let d = Partition::new(lb, ub, density);
            let mut rng = thread_rng();

            for _ in 0..100 {
                let s = d.sample(&mut rng);

                assert!(s < density);
            }
        }

        check(0.0, 5.0, 5);
        check(-5.0, 5.0, 10);
        check(-5.0, 0.0, 5);
    }

    #[test]
    fn test_bounds() {
        fn check(lb: f64, ub: f64, density: usize) {
            let d = Partition::new(lb, ub, density);

            assert_eq!(d.inf().unwrap(), lb);
            assert_eq!(d.sup().unwrap(), ub);

            assert!(!d.contains(ub));
            assert!(d.contains(lb));
            assert!(d.contains((lb + ub) / 2.0));
        }

        check(0.0, 5.0, 5);
        check(-5.0, 5.0, 10);
        check(-5.0, 0.0, 5);
    }

    #[test]
    fn test_range() {
        fn check(lb: f64, ub: f64, density: usize) {
            let d = Partition::new(lb, ub, density);

            assert_eq!(d.range(), 0..density);
        }

        check(0.0, 5.0, 5);
        check(-5.0, 5.0, 10);
        check(-5.0, 0.0, 5);
    }

    #[test]
    fn test_surjection_f64() {
        let d = Partition::new(0.0, 5.0, 6);

        assert_eq!(d.map(-1.0), 0);
        assert_eq!(d.map(0.0), 0);
        assert_eq!(d.map(1.0), 1);
        assert_eq!(d.map(2.0), 2);
        assert_eq!(d.map(3.0), 3);
        assert_eq!(d.map(4.0), 4);
        assert_eq!(d.map(5.0), 5);
        assert_eq!(d.map(6.0), 5);
    }

    #[test]
    fn test_surjection_usize() {
        let d = Partition::new(5.0, 6.0, 2);

        assert_eq!(d.map(0), 0);
        assert_eq!(d.map(1), 1);
        assert_eq!(d.map(2), 1);
    }

    #[test]
    fn test_serialisation() {
        fn check(lb: f64, ub: f64, density: usize) {
            let d = Partition::new(lb, ub, density);

            assert_tokens(
                &d,
                &[
                    Token::Struct {
                        name: "Partition",
                        len: 3,
                    },
                    Token::Str("lb"),
                    Token::F64(lb),
                    Token::Str("ub"),
                    Token::F64(ub),
                    Token::Str("density"),
                    Token::U64(density as u64),
                    Token::StructEnd,
                ],
            );
        }

        check(0.0, 5.0, 5);
        check(-5.0, 5.0, 10);
        check(-5.0, 0.0, 5);
    }
}
