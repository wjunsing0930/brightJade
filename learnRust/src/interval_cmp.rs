
#[derive(Debug, PartialEq)]
struct Interval<T>{
    lower : T, upper : T
}
use std::cmp::{Ordering, PartialOrd};
impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T>{
    fn partial_cmp(&self, other:&Interval<T>) -> Option<Ordering>{
        if self == other{
            Some(Ordering::Equal)
        } else if self.lower >= other.upper{
            Some(Ordering::Greater)
        } else if self.upper <= other.lower{
            Some(Ordering::Less)
        } else{
            None
        }
    }
}

#[test]
fn test_partial_cmp(){
    assert!(Interval{lower:10,upper:20} < Interval{lower:20,upper:40});
    let left  = Interval { lower: 10, upper: 30 };
    let right = Interval { lower: 20, upper: 40 };
    assert!(!(left < right));
    assert!(!(left >= right));
}
