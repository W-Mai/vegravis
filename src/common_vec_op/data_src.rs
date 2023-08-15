use std::cell::RefCell;
use std::rc::Rc;
use crate::interfaces::IDataSource;

pub struct TextDataSrc {
    data: Rc<RefCell<String>>,
}

impl IDataSource<String> for TextDataSrc {
    fn new(code: String) -> Self {
        Self { data: Rc::new(RefCell::new(code)) }
    }

    fn get(&self, _name: &str) -> Option<String> {
        Some(self.data.borrow().clone())
    }
}
