use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use futures::future::BoxFuture;
use tokio::sync::RwLock;

use crate::{
    command::{CommandHandlerManager, CommandMiddleware, NextCommandMiddleware},
    query::{QueryHandler, QueryHandlerManager, QueryMiddleware},
    CommandHandler, DispatchedCommand, DispatchedQuery, NextQueryMiddleware,
};

pub(crate) static BUSSTOP_CMD_QUERY: OnceLock<Arc<Busstop>> = OnceLock::new();

const LOG_TARGET: &str = "bus_stop";

pub struct Busstop {
    command_middlewares: RwLock<HashMap<String, Vec<CommandMiddleware>>>,
    commands: RwLock<HashMap<String, CommandHandlerManager>>,
    queries: RwLock<HashMap<String, QueryHandlerManager>>,
    query_middlewares: RwLock<HashMap<String, Vec<QueryMiddleware>>>,
}

impl Busstop {
    /// Returns the current instance of the bus
    /// A new instance will be created if one does not exist
    /// You can call this method as many times as you like
    pub fn instance() -> Arc<Self> {
        BUSSTOP_CMD_QUERY
            .get_or_init(|| {
                Arc::new(Self {
                    commands: RwLock::new(HashMap::new()),
                    queries: RwLock::new(HashMap::new()),
                    command_middlewares: RwLock::new(HashMap::new()),
                    query_middlewares: RwLock::new(HashMap::new()),
                })
            })
            .clone()
    }

    pub async fn register_command_middleware<C, M>(&self, middleware: M) -> &Self
    where
        M: FnMut(DispatchedCommand, NextCommandMiddleware) -> BoxFuture<'static, DispatchedCommand>
            + Send
            + Sync
            + 'static,
    {
        let name = std::any::type_name::<C>().to_string();

        if self.command_has_handler::<C>().await {
            let mut lock = self.commands.write().await;

            if let Some(manager) = lock.get_mut(&name) {
                manager.next(middleware);
                log::debug!(target: LOG_TARGET, "registered middleware for command {:?}", &name);
            }
        } else {
            let mut lock = self.command_middlewares.write().await;

            log::debug!(target: LOG_TARGET, "queued middleware to be added to command {:?}", &name);
            if let Some(list) = lock.get_mut(&name) {
                list.push(Box::new(middleware));
            } else {
                lock.insert(name, vec![Box::new(middleware)]);
            }
        }

        self
    }

    pub async fn register_query_middleware<T, M>(&self, middleware: M) -> &Self
    where
        M: FnMut(DispatchedQuery, NextQueryMiddleware) -> BoxFuture<'static, DispatchedQuery>
            + Send
            + Sync
            + 'static,
    {
        let name = std::any::type_name::<T>().to_string();

        if self.query_has_handler::<T>().await {
            let mut lock = self.queries.write().await;

            if let Some(manager) = lock.get_mut(&name) {
                manager.next(middleware);
                log::debug!(target: LOG_TARGET, "registered middleware for query for {:?}", &name);
            }
        } else {
            let mut lock = self.query_middlewares.write().await;
            log::debug!(target: LOG_TARGET, "queued middleware to be added to query {:?}", &name);

            if let Some(list) = lock.get_mut(&name) {
                list.push(Box::new(middleware));
            } else {
                lock.insert(name, vec![Box::new(middleware)]);
            }
        }

        self
    }

    /// Register an handler for a command
    pub async fn register_command<C>(&self, handler: impl CommandHandler + 'static) -> &Self {
        let name = std::any::type_name::<C>().to_string();

        let manager = CommandHandlerManager::new(handler);

        let mut lock = self.command_middlewares.write().await;
        if let Some(middlewares) = lock.remove(&name) {
            for cm in middlewares.into_iter() {
                manager.next(cm);
            }
        }
        drop(lock);

        let mut lock = self.commands.write().await;
        log::debug!(target: LOG_TARGET, "registered command handler {:?} for  {:?}", manager.name(), &name);
        lock.insert(name, manager);

        self
    }

    /// Checks if a command has a register handler
    pub async fn command_has_handler<C>(&self) -> bool {
        let name = std::any::type_name::<C>().to_string();
        let lock = self.commands.read().await;

        lock.contains_key(&name)
    }

    /// Register an handler for a command
    pub async fn register_query<T>(&self, handler: impl QueryHandler + 'static) -> &Self {
        let name = std::any::type_name::<T>().to_string();

        log::debug!(target: LOG_TARGET, "registered query handler {:?} for  {:?}", handler.query_handler_name(), &name);

        let manager = QueryHandlerManager::new(handler);

        let mut lock = self.query_middlewares.write().await;
        if let Some(middlewares) = lock.remove(&name) {
            for qm in middlewares.into_iter() {
                manager.next(qm);
            }
        }
        drop(lock);

        let mut lock = self.queries.write().await;
        lock.insert(name, manager);

        self
    }

    /// Checks if a query has a registered handler
    pub async fn query_has_handler<Q>(&self) -> bool {
        let name = std::any::type_name::<Q>().to_string();
        let lock = self.queries.read().await;

        lock.contains_key(&name)
    }

    /// Dispatches a command event
    pub async fn dispatch_command<T: Send + Sync + 'static>(&self, command: T) -> bool {
        let name = std::any::type_name::<T>().to_string();

        log::debug!(target: LOG_TARGET, "dispatching command: {:?}", &name);
        let dispatched_command = DispatchedCommand::new(Box::new(command), &name);

        let lock = self.commands.read().await;
        if let Some(handler) = lock.get(&name) {
            let result = handler.handle(dispatched_command).await;
            log::debug!(target: LOG_TARGET, "command: {:?} was handled by: {:?}", &name, handler.name());
            result.handled
        } else {
            log::debug!(target: LOG_TARGET, "command: {:?} was not handled", &name);
            dispatched_command.handled
        }
    }

    /// Dispatches a query event
    pub async fn dispatch_query<Q: Send + Sync + 'static>(&self, query: Q) -> DispatchedQuery {
        let name = std::any::type_name::<Q>().to_string();

        log::debug!(target: LOG_TARGET, "dispatching query: {:?}", &name);
        let dispatched_query = DispatchedQuery::new(Box::new(query), &name);

        let lock = self.queries.read().await;
        if let Some(handler) = lock.get(&name) {
            let result = handler.handle(dispatched_query).await;
            log::debug!(target: LOG_TARGET, "query: {:?} was handled by: {:?}", &name, handler.name());
            result
        } else {
            log::debug!(target: LOG_TARGET, "query: {:?} was not handled", &name);
            dispatched_query
        }
    }
}
