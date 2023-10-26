use std::any::Any;

pub struct DispatchedCommand(Box<dyn Any + Send + Sync>);

impl DispatchedCommand {
    pub(crate) fn new(inner: Box<dyn Any + Send + Sync>) -> Self {
        Self(inner)
    }

    pub fn the_command<T: 'static>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }
}
