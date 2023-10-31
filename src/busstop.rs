use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use tokio::sync::RwLock;

use crate::{query::QueryHandler, CommandHandler, DispatchedCommand, DispatchedQuery};

pub(crate) static BUSSTOP_CMD_QUERY: OnceLock<Arc<Busstop>> = OnceLock::new();

const LOG_TARGET: &str = "bus_stop";

pub struct Busstop {
    commands: RwLock<HashMap<String, Box<dyn CommandHandler>>>,
    queries: RwLock<HashMap<String, Box<dyn QueryHandler>>>,
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
                })
            })
            .clone()
    }

    /// Register an handler for a command
    pub async fn register_command<C>(&self, handler: impl CommandHandler + 'static) -> &Self {
        let name = std::any::type_name::<C>().to_string();
        let mut lock = self.commands.write().await;

        log::debug!(target: LOG_TARGET, "registered command handler {:?} for  {:?}", handler.command_handler_name(), &name);

        lock.insert(name, Box::new(handler));

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
        let mut lock = self.queries.write().await;

        log::debug!(target: LOG_TARGET, "registered query handler {:?} for  {:?}", handler.query_handler_name(), &name);

        lock.insert(name, Box::new(handler));

        self
    }

    /// Checks if a query has a registered handler
    pub async fn query_has_handler<Q>(&self) -> bool {
        let name = std::any::type_name::<Q>().to_string();
        let lock = self.queries.read().await;

        lock.contains_key(&name)
    }

    /// Dispatches a command event
    pub async fn dispatch_command<T: Send + Sync + 'static>(
        &self,
        command: T,
    ) -> DispatchedCommand {
        let name = std::any::type_name::<T>().to_string();

        log::debug!(target: LOG_TARGET, "dispatching command: {:?}", &name);
        let dispatched_command = DispatchedCommand::new(Box::new(command));

        let lock = self.commands.read().await;
        if let Some(handler) = lock.get(&name) {
            let mut result = handler.handle_command(dispatched_command).await;
            result.handled = true;

            log::debug!(target: LOG_TARGET, "command: {:?} was handled by: {:?}", &name, handler.command_handler_name());
            result
        } else {
            log::debug!(target: LOG_TARGET, "command: {:?} was not handled", &name);
            dispatched_command
        }
    }

    /// Dispatches a query event
    pub async fn dispatch_query<Q: Send + Sync + 'static>(&self, query: Q) -> DispatchedQuery {
        let name = std::any::type_name::<Q>().to_string();

        log::debug!(target: LOG_TARGET, "dispatching query: {:?}", &name);
        let dispatched_query = DispatchedQuery::new(Box::new(query));

        let lock = self.queries.read().await;
        if let Some(handler) = lock.get(&name) {
            let mut result = handler.handle_query(dispatched_query).await;
            result.handled = true;

            log::debug!(target: LOG_TARGET, "query: {:?} was handled by: {:?}", &name, handler.query_handler_name());
            result
        } else {
            log::debug!(target: LOG_TARGET, "query: {:?} was not handled", &name);
            dispatched_query
        }
    }
}
