use std::{any::Any, cell::OnceCell};

use crate::DispatchableQuery;

#[derive(Debug)]
pub struct DispatchedQuery {
    query: Option<Box<dyn Any + Send + Sync>>,
    value: OnceCell<(Box<dyn Any + Send + Sync>, String)>,
    name: String,
    pub(crate) handled: bool,
}

impl DispatchedQuery {
    pub(crate) fn new(query: Box<dyn Any + Send + Sync>, name: &str) -> Self {
        Self {
            query: Some(query),
            value: OnceCell::new(),
            handled: false,
            name: name.to_string(),
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

    /// Returns a mutable reference to the query
    pub fn the_query_mut<T: 'static>(&mut self) -> Option<&mut T> {
        if let Some(query) = &mut self.query {
            query.downcast_mut()
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
        let x = std::any::type_name::<V>();
        if self.value.set((Box::new(value), x.to_string())).is_err() {
            tracing::error!(target: "dispatched query", "value can only be set once. Query: {}", &self.name);
        }
    }

    /// Returns the value set by the handler of the query
    pub fn value<T: 'static>(&self) -> Option<&T> {
        if let Some(v) = self.value.get() {
            v.0.downcast_ref()
        } else {
            None
        }
    }

    /// Returns the query's result value
    pub fn take_value<T: 'static>(&mut self) -> Option<Box<T>> {
        if let Some(v) = self.value.take() {
            return v.0.downcast().ok();
        }

        None
    }

    /// Returns true if the query was handled
    pub fn handled(&self) -> bool {
        self.handled
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    /// Compares the type of Q with this dispatched query type
    pub fn is<Q>(&self) -> bool {
        std::any::type_name::<Q>() == self.name()
    }

    /// Compares the type of the value with the type of T
    pub fn value_type_is<T>(&self) -> bool {
        if let Some((_, name)) = self.value.get() {
            std::any::type_name::<T>() == name
        } else {
            false
        }
    }
}

impl<Q: DispatchableQuery + 'static> From<Q> for DispatchedQuery {
    fn from(value: Q) -> Self {
        let name = std::any::type_name::<Q>().to_string();
        Self::new(Box::new(value), &name)
    }
}
