use std::{any::Any, cell::OnceCell};

#[derive(Debug)]
pub struct DispatchedQuery {
    query: Option<Box<dyn Any + Send + Sync>>,
    value: Option<OnceCell<Box<dyn Any + Send + Sync>>>,
    pub(crate) handled: bool,
}

impl DispatchedQuery {
    pub(crate) fn new(query: Box<dyn Any + Send + Sync>) -> Self {
        Self {
            query: Some(query),
            value: Some(OnceCell::new()),
            handled: false,
        }
    }

    /// Returns a reference (the real query) of the dispatched query
    pub fn the_query<T: 'static>(&self) -> Option<&T> {
        if let Some(query) = &self.query {
            query.downcast_ref()
        } else {
            None
        }
    }

    /// Returns the dispatched query
    /// Subsequent call to this method or `the_query` will return none
    pub fn take_query<T: 'static>(&mut self) -> Option<Box<T>> {
        if let Some(query) = self.query.take() {
            query.downcast().ok()
        } else {
            None
        }
    }

    /// Sets the value that will be returned to the dispatcher
    pub fn set_value<V: Send + Sync + 'static>(&self, value: V) {
        if let Some(v) = &self.value {
            _ = v.set(Box::new(value));
        }
    }

    /// Returns the value set by the handler of the query
    pub fn value<T: 'static>(&self) -> Option<&T> {
        if let Some(v) = &self.value {
            v.get().unwrap().downcast_ref()
        } else {
            None
        }
    }

    /// Returns the query's result value
    pub fn take_value<T: 'static>(&mut self) -> Option<Box<T>> {
        if let Some(v) = self.value.take() {
            return v.into_inner().unwrap().downcast().ok();
        }

        None
    }

    /// Returns true if the query was handled
    pub fn handled(&self) -> bool {
        self.handled
    }
}
