use crate::prelude::*;

macro_rules! stripped {
    (* $($rest: tt)*) => {
        $($rest)*
    };
    (|| $($rest: tt)*) => {
        $($rest)*
    };
    (&& $($rest: tt)*) => {
        $($rest)*
    };
}

macro_rules! impl_tuple {
    ($n:literal; $(($tp:ident, $vp:ident)::$i:tt),+) => {
        impl<$($tp: Space),+> Space for ($($tp),+) {
            type Value = ($($tp::Value),+);

            fn is_empty(&self) -> bool {
                stripped!($(|| self.$i.is_empty())+)
            }

            fn contains(&self, val: &Self::Value) -> bool {
                stripped!($(&& self.$i.contains(&val.$i))+)
            }
        }

        impl<$($tp: FiniteSpace),+> FiniteSpace for ($($tp),+) {
            fn cardinality(&self) -> usize {
                stripped!($(* self.$i.cardinality())+)
            }
        }

        // impl<$($tp: Union),+> Union for ($($tp),+) {
            // fn union(self, other: &Self) -> Self {
                // ($(self.$i.union(&other.$i)),+).into()
            // }
        // }

        // impl<$($tp: Intersect),+> Intersect for ($($tp),+) {
            // fn intersect(self, other: &Self) -> Self {
                // ($(self.$i.intersect(&other.$i)),+).into()
            // }
        // }
    }
}

impl_tuple!(2; (D1, X1)::0, (D2, X2)::1);
impl_tuple!(3; (D1, X1)::0, (D2, X2)::1, (D3, X3)::2);
impl_tuple!(4; (D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3);
impl_tuple!(5; (D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4);
impl_tuple!(6; (D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5);
impl_tuple!(7; (D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5, (D7, X7)::6);
impl_tuple!(8; (D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5, (D7, X7)::6, (D8, X8)::7);
impl_tuple!(9; (D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5, (D7, X7)::6, (D8, X8)::7, (D9, X9)::8);
impl_tuple!(10; (D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5, (D7, X7)::6, (D8, X8)::7, (D9, X9)::8, (D10, X10)::9);
impl_tuple!(11; (D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5, (D7, X7)::6, (D8, X8)::7, (D9, X9)::8, (D10, X10)::9, (D11, X11)::10);
impl_tuple!(12; (D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5, (D7, X7)::6, (D8, X8)::7, (D9, X9)::8, (D10, X10)::9, (D11, X11)::10, (D12, X12)::11);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intervals::Interval;

    #[test]
    fn test_cardinality() {
        let a = Interval::lorc_unchecked(0usize, 2usize);
        let b = a.clone();

        assert_eq!((a, b).cardinality(), 4);
    }
}
