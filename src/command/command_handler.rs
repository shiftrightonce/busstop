use super::dispatched_command::DispatchedCommand;

/// A command's handler must implement this trait
#[async_trait::async_trait]
pub trait CommandHandler: Send + Sync {
    /// This method is call to handle the dispatched command
    async fn handle_command(&self, command: DispatchedCommand);

    /// A unique name for this handler
    /// By default, the path to the type is used
    fn command_handler_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}
