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

    /// Returns the inner (the real query) of the dispatched query
    pub fn the_query<T: 'static>(&self) -> Option<&T> {
        self.query.downcast_ref()
    }

    /// Sets the value that will be returned to the dispatcher
    pub fn set_value<V: Send + Sync + 'static>(&self, value: V) {
        _ = self.value.set(Box::new(value));
    }

    /// Returns the value set by the handler of the query
    pub fn value<T: 'static>(&self) -> Option<&T> {
        self.value.get().unwrap().downcast_ref()
    }
}
