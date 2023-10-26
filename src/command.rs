mod command_handler;
mod dispatched_command;

pub use command_handler::CommandHandler;
pub use dispatched_command::DispatchedCommand;

use crate::Busstop;

#[async_trait::async_trait]
pub trait DispatchableCommand: Send + Sync {
    async fn dispatch_command(self)
    where
        Self: Sized + 'static,
    {
        Busstop::instance().dispatch_command(self).await;
    }

    async fn command_handler<H: CommandHandler + Default + 'static>()
    where
        Self: Sized,
    {
        Busstop::instance()
            .register_command::<Self>(H::default())
            .await;
    }

    async fn register_command_handler<H: CommandHandler + 'static>(handler: H)
    where
        Self: Sized,
    {
        Busstop::instance().register_command::<Self>(handler).await;
    }
}
