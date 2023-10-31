use std::any::Any;

#[derive(Debug)]
pub struct DispatchedCommand {
    inner: Box<dyn Any + Send + Sync>,
    pub(crate) handled: bool,
}

impl DispatchedCommand {
    pub(crate) fn new(inner: Box<dyn Any + Send + Sync>) -> Self {
        Self {
            inner,
            handled: false,
        }
    }

    /// Returns the inner (the real command) of the dispatched command
    pub fn the_command<T: 'static>(&self) -> Option<&T> {
        self.inner.downcast_ref()
    }

    pub fn the_command_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.inner.downcast_mut()
    }

    pub fn handled(&self) -> bool {
        self.handled
    }
}
