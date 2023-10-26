use std::{any::Any, cell::OnceCell};

#[derive(Debug)]
pub struct DispatchedQuery {
    query: Box<dyn Any + Send + Sync>,
    value: OnceCell<Box<dyn Any + Send + Sync>>,
}

impl DispatchedQuery {
    pub(crate) fn new(query: Box<dyn Any + Send + Sync>) -> Self {
        Self {
            query,
            value: OnceCell::new(),
        }
    }

    pub fn the_query<T: 'static>(&self) -> Option<&T> {
        self.query.downcast_ref()
    }

    pub fn set_value<V: Send + Sync + 'static>(&self, value: V) {
        _ = self.value.set(Box::new(value));
    }

    pub fn value<T: 'static>(&self) -> Option<&T> {
        self.value.get().unwrap().downcast_ref()
    }
}
