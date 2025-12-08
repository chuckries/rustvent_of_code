// struct that provides an impl or Ord to anything that only has PartialOrd. 
// panics if PartialOrd fails
// this is good for f32, f64 in sorting scenarios
#[derive(Copy, Clone)]
pub struct OrdWrapper<T>(pub T);

impl<T: PartialEq> PartialEq for OrdWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    fn ne(&self, other: &Self) -> bool {
        self.0.ne(&other.0)
    }
}

impl<T: PartialEq> Eq for OrdWrapper<T> { }

impl<T: PartialOrd> PartialOrd for OrdWrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }

    fn ge(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }

    fn gt(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }

    fn le(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }

    fn lt(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }
}

impl<T: PartialOrd> Ord for OrdWrapper<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}