pub trait Equivalence {
    fn equivalent(self, other: Self) -> bool;
}

#[macro_export]
macro_rules! assert_equivalent {
    ($l:expr, $r:expr) => {
        if !Equivalence::equivalent($l, $r) {
            panic!("Not equivalent!\nleft :{:#?}\nright:{:#?}", $l, $r)
        }
    };
}

macro_rules! implement_from_eq {
    ($name:ty) => {
        impl Equivalence for $name {
            fn equivalent(self, other: Self) -> bool {
                self == other
            }
        }
    };
}

implement_from_eq!(u8);
implement_from_eq!(u16);
implement_from_eq!(u32);
implement_from_eq!(u64);
implement_from_eq!(i8);
implement_from_eq!(usize);
implement_from_eq!(i16);
implement_from_eq!(i32);
implement_from_eq!(i64);
implement_from_eq!(isize);
implement_from_eq!(String);
implement_from_eq!(char);

impl<T: Equivalence> Equivalence for Option<T> {
    fn equivalent(self, other: Self) -> bool {
        match (self, other) {
            (None, None) => true,
            (Some(s), Some(o)) => s.equivalent(o),
            _ => false,
        }
    }
}

const fn count_elements<const N: usize>(_elements: [(); N]) -> usize {
    N
}

macro_rules! replace {
    ($_t:tt,$sub:expr) => {
        $sub
    };
}

macro_rules! count_tts {
    ($($something:tt)*) => {
        count_elements( [$(replace!( $something, () ) ),* ] )
    };
}

//macro_rules! implement_tuple{
//    ($($name:ident),+) => {
//        impl<$($name,)+> Equivalence for ($($name,)+)
//            where
//                $($name : Equivalence,)+
//        {
//            fn equivalent(self, other: Self) -> bool {
//                for i in 0..count_tts!($($name)+){
//                    if !Equivalence::equivalent(self.get(i),other.get(i)){
//                        return false;
//                    }
//                }
//                return true;
//
//            }
//        }
//    };
//}
//
//implement_tuple!(A1, A2);
//

impl<A: Equivalence, B: Equivalence> Equivalence for (A, B) {
    fn equivalent(self, other: Self) -> bool {
        let (a1, a2) = self;
        let (b1, b2) = other;
        Equivalence::equivalent(a1, b1) & Equivalence::equivalent(a2, b2)
    }
}

impl<T: Equivalence> Equivalence for Box<T> {
    fn equivalent(self, other: Self) -> bool {
        (*self).equivalent(*other)
    }
}

impl<T> Equivalence for Vec<T>
where
    T: Equivalence,
{
    fn equivalent(self, other: Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for (l, r) in std::iter::zip(self, other) {
            if !Equivalence::equivalent(l, r) {
                return false;
            }
        }
        return true;
    }
}
