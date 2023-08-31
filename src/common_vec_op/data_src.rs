use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::interfaces::IDataSource;

#[derive(PartialEq)]
pub struct TextDataSrc {
    data: Rc<RefCell<String>>,
}

impl Clone for TextDataSrc {
    fn clone(&self) -> Self {
        TextDataSrc {
            data: Rc::new(RefCell::new(self.data.borrow().clone()))
        }
    }
}

impl IDataSource for TextDataSrc {
    type ST = String;

    fn new(code: Self::ST) -> Self {
        Self { data: Rc::new(RefCell::new(code)) }
    }

    fn get(&self, _name: &str) -> Option<Self::ST> {
        Some(self.data.borrow().clone())
    }

    fn get_ref(&self, _name: &str) -> Option<RefMut<Self::ST>> {
        Some(self.data.borrow_mut())
    }
}
