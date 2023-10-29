mod command_handler;
mod dispatched_command;

pub use command_handler::CommandHandler;
pub use dispatched_command::DispatchedCommand;

use crate::Busstop;

/// A type that can be used as a command can implement this trait.
/// Implementing this trait makes it easy to register an handler
/// and to dispatch the command.
#[async_trait::async_trait]
pub trait DispatchableCommand: Send + Sync {
    /// Dispatch the command
    async fn dispatch_command(self)
    where
        Self: Sized + 'static,
    {
        Busstop::instance().dispatch_command(self).await;
    }

    /// Register this handler for this command
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

    /// Register the instance as the handler for this command
    async fn register_command_handler<H: CommandHandler + 'static>(handler: H)
    where
        Self: Sized,
    {
        Busstop::instance().register_command::<Self>(handler).await;
    }

    /// Register the instance as the soft handler for this command
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
