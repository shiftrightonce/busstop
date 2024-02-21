use std::any::Any;

#[derive(Debug)]
pub struct DispatchedCommand {
    inner: Option<Box<dyn Any + Send + Sync>>,
    pub(crate) handled: bool,
}

impl DispatchedCommand {
    pub(crate) fn new(inner: Box<dyn Any + Send + Sync>) -> Self {
        Self {
            inner: Some(inner),
            handled: false,
        }
    }

    /// Returns a reference to (the real command)  the dispatched command
    pub fn the_command<T: 'static>(&self) -> Option<&T> {
        if let Some(inner) = &self.inner {
            inner.downcast_ref()
        } else {
            None
        }
    }

    /// Returns a mutable reference to (the real command)  the dispatched command
    pub fn the_command_mut<T: 'static>(&mut self) -> Option<&mut T> {
        if let Some(inner) = &mut self.inner {
            inner.downcast_mut()
        } else {
            None
        }
    }

    /// Returns (the real command)  the dispatched command
    /// Subsequent to this method or `the_command_mut` and `the_command` will returned none
    pub fn take_command<T: 'static>(&mut self) -> Option<Box<T>> {
        if let Some(inner) = self.inner.take() {
            return inner.downcast().ok();
        }
        None
    }

    /// Returns true if the command was handled
    pub fn handled(&self) -> bool {
        self.handled
    }
}
