use std::any::Any;
use std::cmp::Ordering;

const NOT_MATCH_MSG: &str = "Type is not matched";

#[derive(Debug)]
pub struct AnyData {
    data: Box<dyn Any>,
}

#[allow(dead_code)]
impl AnyData {
    pub fn new<T: Any>(data: T) -> Self {
        Self {
            data: Box::new(data)
        }
    }

    pub fn cast<T: Any>(self) -> T {
        *self.data.downcast().expect(NOT_MATCH_MSG)
    }

    pub fn cast_ref<T: Any>(&self) -> &T {
        self.data.downcast_ref().expect(NOT_MATCH_MSG)
    }

    pub fn cast_mut<T: Any>(&mut self) -> &mut T {
        self.data.downcast_mut().expect(NOT_MATCH_MSG)
    }

    pub fn convert_vec<T: Any + Clone>(data: Vec<T>) -> Vec<AnyData> {
        data.iter().cloned().map(|v| {
            AnyData::new(v)
        }).collect()
    }
}

#[allow(unused)]
impl AnyData {
    pub fn clone<T: Any + Clone>(&self) -> Self {
        AnyData::new(self.cast_ref::<T>().clone())
    }

    pub fn equal<T1: Any + PartialEq + PartialEq<T2>, T2: Any + PartialEq>(&self, another: &AnyData) -> bool {
        self.cast_ref::<T1>() == another.cast_ref::<T2>()
    }

    pub fn compare<T1: Any + PartialOrd + PartialOrd<T2>, T2: Any + PartialOrd>(&self, another: &AnyData) -> Ordering {
        let lhs = self.cast_ref::<T1>();
        let rhs = another.cast_ref::<T2>();
        if lhs > rhs {
            Ordering::Greater
        } else if lhs < rhs {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
