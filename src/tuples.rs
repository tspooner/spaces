use crate::prelude::*;

macro_rules! stripped {
    (+ $($rest: tt)*) => {
        $($rest)*
    };
    (* $($rest: tt)*) => {
        $($rest)*
    };
    (&& $($rest: tt)*) => {
        $($rest)*
    };
}

macro_rules! impl_tuple {
    ($(($tp:ident, $vp:ident)::$i:tt),+) => {
        impl<$($tp: Space),+> Space for ($($tp),+) {
            type Value = ($($tp::Value),+);

            fn dim(&self) -> Dim {
                stripped!($(+ self.$i.dim())+)
            }

            fn card(&self) -> Card {
                stripped!($(* self.$i.card())+)
            }

            fn contains(&self, val: &Self::Value) -> bool {
                stripped!($(&& self.$i.contains(&val.$i))+)
            }
        }

        impl<$($tp: Union),+> Union for ($($tp),+) {
            fn union(self, other: &Self) -> Self {
                ($(self.$i.union(&other.$i)),+).into()
            }
        }

        impl<$($tp: Intersection),+> Intersection for ($($tp),+) {
            fn intersect(self, other: &Self) -> Self {
                ($(self.$i.intersect(&other.$i)),+).into()
            }
        }

        impl<$($tp, $vp),+> Project<($($vp),+), ($($tp::Value),+)> for ($($tp),+)
        where
            $($tp: Space + Project<$vp, <$tp as Space>::Value>),+
        {
            fn project(&self, val: ($($vp),+)) -> ($($tp::Value),+) {
                ($(self.$i.project(val.$i),)+)
            }
        }
    }
}

impl_tuple!((D1, X1)::0, (D2, X2)::1);
impl_tuple!((D1, X1)::0, (D2, X2)::1, (D3, X3)::2);
impl_tuple!((D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3);
impl_tuple!((D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4);
impl_tuple!((D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5);
impl_tuple!((D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5, (D7, X7)::6);
impl_tuple!((D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5, (D7, X7)::6, (D8, X8)::7);
impl_tuple!((D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5, (D7, X7)::6, (D8, X8)::7, (D9, X9)::8);
impl_tuple!((D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5, (D7, X7)::6, (D8, X8)::7, (D9, X9)::8, (D10, X10)::9);
impl_tuple!((D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5, (D7, X7)::6, (D8, X8)::7, (D9, X9)::8, (D10, X10)::9, (D11, X11)::10);
impl_tuple!((D1, X1)::0, (D2, X2)::1, (D3, X3)::2, (D4, X4)::3, (D5, X5)::4, (D6, X6)::5, (D7, X7)::6, (D8, X8)::7, (D9, X9)::8, (D10, X10)::9, (D11, X11)::10, (D12, X12)::11);

#[cfg(test)]
mod tests {
    use crate::Interval;
    use super::*;

    #[test]
    fn test_dim() {
        assert_eq!((0..2, 0..2).dim(), Dim::Finite(2));
    }

    #[test]
    fn test_card() {
        assert_eq!((0..2, 0..2).card(), Card::Finite(4));
    }

    #[test]
    fn test_surjection() {
        let ps = (Interval::bounded(0.0, 5.0), Interval::bounded(1.0, 2.0));

        assert_eq!(ps.project((6.0, 0.0)), (5.0, 1.0));
        assert_eq!(ps.project((2.5, 1.5)), (2.5, 1.5));
        assert_eq!(ps.project((-1.0, 10.0)), (0.0, 2.0));
    }
}
