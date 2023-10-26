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

    pub async fn register_command<T>(&self, handler: impl CommandHandler + 'static) -> &Self {
        let name = std::any::type_name::<T>().to_string();
        let mut lock = self.commands.write().await;

        log::debug!(target: LOG_TARGET, "registered command handler {:?} for  {:?}", handler.command_handler_name(), &name);

        lock.insert(name, Box::new(handler));

        self
    }

    pub async fn register_query<T>(&self, handler: impl QueryHandler + 'static) -> &Self {
        let name = std::any::type_name::<T>().to_string();
        let mut lock = self.queries.write().await;

        log::debug!(target: LOG_TARGET, "registered query handler {:?} for  {:?}", handler.query_handler_name(), &name);

        lock.insert(name, Box::new(handler));

        self
    }

    pub async fn dispatch_command<T: Send + Sync + 'static>(&self, command: T) {
        let name = std::any::type_name::<T>().to_string();

        log::debug!(target: LOG_TARGET, "dispatching command: {:?}", &name);

        let lock = self.commands.read().await;
        if let Some(handler) = lock.get(&name) {
            handler
                .handle_command(DispatchedCommand::new(Box::new(command)))
                .await;

            log::debug!(target: LOG_TARGET, "command: {:?} was handled by: {:?}", &name, handler.command_handler_name());
        } else {
            log::debug!(target: LOG_TARGET, "command: {:?} was not handled", &name);
        }
    }

    pub async fn dispatch_query<Q: Send + Sync + 'static>(
        &self,
        query: Q,
    ) -> Option<DispatchedQuery> {
        let name = std::any::type_name::<Q>().to_string();

        log::debug!(target: LOG_TARGET, "dispatching query: {:?}", &name);

        let lock = self.queries.read().await;
        if let Some(handler) = lock.get(&name) {
            let result = handler
                .handle_query(DispatchedQuery::new(Box::new(query)))
                .await;

            log::debug!(target: LOG_TARGET, "query: {:?} was handled by: {:?}", &name, handler.query_handler_name());
            Some(result)
        } else {
            log::debug!(target: LOG_TARGET, "query: {:?} was not handled", &name);
            None
        }
    }
}
