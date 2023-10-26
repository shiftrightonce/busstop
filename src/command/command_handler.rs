use super::dispatched_command::DispatchedCommand;

#[async_trait::async_trait]
pub trait CommandHandler: Send + Sync {
    async fn handle_command(&self, command: DispatchedCommand);

    fn command_handler_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}
