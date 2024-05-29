mod command_handler;
mod dispatched_command;

use std::sync::Arc;

pub use command_handler::CommandHandler;
pub use dispatched_command::DispatchedCommand;
use futures::future::BoxFuture;
use simple_middleware::{Manager as MiddlewareManager, Next};

use crate::Busstop;

/// Next middleware to call. Send argument pass to all commands' middlewares
pub type NextCommandMiddleware = Next<DispatchedCommand, DispatchedCommand>;

/// Command middleware type
pub type CommandMiddleware = Box<
    dyn FnMut(DispatchedCommand, NextCommandMiddleware) -> BoxFuture<'static, DispatchedCommand>
        + Send
        + Sync,
>;

/// A type that can be used as a command can implement this trait.
/// Implementing this trait makes it easy to register an handler
/// and to dispatch the command.
#[async_trait::async_trait]
pub trait DispatchableCommand: Send + Sync {
    /// Dispatch the command
    async fn dispatch_command(self) -> bool
    where
        Self: Sized + 'static,
    {
        Busstop::instance().dispatch_command(self).await
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

    /// Register a middleware on this dispatchable command
    async fn command_middleware<M: 'static>(middleware: M)
    where
        Self: Sized,
        M: FnMut(
                DispatchedCommand,
                Next<DispatchedCommand, DispatchedCommand>,
            ) -> BoxFuture<'static, DispatchedCommand>
            + Send
            + Sync,
    {
        Busstop::instance()
            .register_command_middleware::<Self, M>(middleware)
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

/// Manages the middlewares for the current command handler
pub struct CommandHandlerManager {
    name: String,
    middleware: MiddlewareManager<DispatchedCommand, DispatchedCommand>,
}

impl CommandHandlerManager {
    /// Create a new instance
    pub fn new(handler: impl CommandHandler + 'static) -> Self {
        let handler = Arc::new(Box::new(handler));
        Self {
            name: handler.command_handler_name().to_string(),
            middleware: MiddlewareManager::last(move |dispatched, _| {
                let instance = handler.clone();
                Box::pin(async move { instance.clone().handle_command(dispatched).await })
            }),
        }
    }

    /// The name of the command
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn next<M>(&self, middleware: M) -> &Self
    where
        M: FnMut(DispatchedCommand, NextCommandMiddleware) -> BoxFuture<'static, DispatchedCommand>
            + Send
            + 'static,
    {
        self.middleware.next(middleware);
        self
    }

    pub async fn handle(&self, dispatched: DispatchedCommand) -> DispatchedCommand {
        let mut result = self.middleware.send(dispatched).await;
        result.handled = true;

        result
    }

    pub async fn handle_command<C: Send + Sync + 'static>(&self, command: C) -> DispatchedCommand {
        self.handle(DispatchedCommand::new(
            Box::new(command),
            std::any::type_name::<C>(),
        ))
        .await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Cmd;
    impl DispatchableCommand for Cmd {}

    #[derive(Default)]
    struct CmdHandler;

    #[async_trait::async_trait]
    impl CommandHandler for CmdHandler {
        async fn handle_command(&self, c: DispatchedCommand) -> DispatchedCommand {
            c
        }
    }

    #[tokio::test]
    async fn test_command_handler_manager() {
        let manager = CommandHandlerManager::new(CmdHandler);

        manager.next(|c, n| Box::pin(async move { n.call(c).await }));
        manager.next(|c, n| Box::pin(async move { n.call(c).await }));

        assert_eq!(manager.handle_command(Cmd).await.handled(), true)
    }
}
