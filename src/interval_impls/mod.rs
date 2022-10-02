use intervals::Interval;

macro_rules! impl_union_pair {
    ($v:ident; $a:ty, $b:ty) => {
        impl<V: PartialOrd> ops::Union<$b> for $a {
            type Output = crate::ops::UnionPair<$a, $b>;

            fn union(self, rhs: $b) -> Self::Output { crate::ops::UnionPair(self, rhs) }
        }
    };
}

impl<L, R, LL, RR> crate::ops::Intersection<Interval<LL, RR>> for Interval<L, R>
where
    L: intervals::bounds::Pinch<LL>,
    R: intervals::bounds::Pinch<RR, Value = L::Value>,

    L::Value: PartialEq,

    LL: intervals::bounds::Bound,
    RR: intervals::bounds::Bound<Value = LL::Value>,

    Interval<L, R>: crate::Space<Value = L::Value>,
    Interval<LL, RR>: crate::Space<Value = LL::Value>,

    intervals::IntersectionOf<L, R, LL, RR>: crate::Space<Value = L::Value>,
{
    type Output = intervals::IntersectionOf<L, R, LL, RR>;

    fn intersection(self, rhs: Interval<LL, RR>) -> Option<Self::Output> {
        let x_cap_y = self.intersect(rhs);

        if x_cap_y.is_degenerate() {
            None
        } else {
            Some(x_cap_y)
        }
    }
}

mod bounded;
mod partial;
mod partitions;
mod unbounded;
