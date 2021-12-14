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
    ($n:literal; $(($tp:ident, $vp:ident)::$i:tt),+) => {
        impl<$($tp: Space),+> Space for ($($tp),+) {
            const DIM: usize = $n;

            type Value = ($($tp::Value),+);

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

        impl<$($tp: Intersect),+> Intersect for ($($tp),+) {
            fn intersect(self, other: &Self) -> Self {
                ($(self.$i.intersect(&other.$i)),+).into()
            }
        }
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

    type S = (::std::ops::Range<usize>, ::std::ops::Range<usize>);

    #[test]
    fn test_dim() {
        assert_eq!(S::DIM, 2);
    }

    #[test]
    fn test_card() {
        assert_eq!((0..2, 0..2).card(), Card::Finite(4));
    }
}
