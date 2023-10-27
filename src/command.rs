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

    /// Register this handler if the command does not have an existing handler
    async fn soft_command_handler<H: CommandHandler + Default + 'static>()
    where
        Self: Sized,
    {
        let bus = Busstop::instance();
        if !bus.command_has_handler::<Self>().await {
            bus.register_command::<Self>(H::default()).await;
        }
    }

    async fn register_command_handler<H: CommandHandler + 'static>(handler: H)
    where
        Self: Sized,
    {
        Busstop::instance().register_command::<Self>(handler).await;
    }

    async fn register_soft_command_handler<H: CommandHandler + 'static>(handler: H)
    where
        Self: Sized,
    {
        let bus = Busstop::instance();
        if !bus.command_has_handler::<Self>().await {
            bus.register_command::<Self>(handler).await;
        }
    }
}
