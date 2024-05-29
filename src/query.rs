mod dispatched_query;
mod query_handler;

use std::sync::Arc;

pub use dispatched_query::DispatchedQuery;
use futures::future::BoxFuture;
pub use query_handler::QueryHandler;
use simple_middleware::{Manager as MiddlewareManager, Next};

use crate::Busstop;

pub type NextQueryMiddleware = Next<DispatchedQuery, DispatchedQuery>;

pub type QueryMiddleware = Box<
    dyn FnMut(DispatchedQuery, NextQueryMiddleware) -> BoxFuture<'static, DispatchedQuery>
        + Send
        + Sync,
>;

/// A type that can be used as a query subject can implement
/// this trait. Implementing this trait makes it easy to register an handler
/// and to dispatch the query.
#[async_trait::async_trait]
pub trait DispatchableQuery: Send + Sync {
    /// Dispatch the query event
    async fn dispatch_query(self) -> DispatchedQuery
    where
        Self: Sized + 'static,
    {
        Busstop::instance().dispatch_query(self).await
    }

    /// Register a handler for for this query
    async fn query_handler<H: QueryHandler + Default + 'static>()
    where
        Self: Sized,
    {
        Busstop::instance()
            .register_query::<Self>(H::default())
            .await;
    }

    async fn query_middleware<M: 'static>(middleware: M)
    where
        Self: Sized,
        M: FnMut(DispatchedQuery, NextQueryMiddleware) -> BoxFuture<'static, DispatchedQuery>
            + Send
            + Sync,
    {
        Busstop::instance()
            .register_query_middleware::<Self, M>(middleware)
            .await;
    }

    /// Register this handler if the query does not have an existing handler
    async fn soft_query_handler<H: QueryHandler + Default + 'static>()
    where
        Self: Sized,
    {
        let bus = Busstop::instance();
        if !bus.query_has_handler::<Self>().await {
            bus.register_query::<Self>(H::default()).await;
        }
    }

    /// Register the current handler instance as the handler of this
    /// query
    async fn register_query_handler<H: QueryHandler + 'static>(handler: H)
    where
        Self: Sized,
    {
        Busstop::instance().register_query::<Self>(handler).await;
    }

    /// Register the current handler instance as the soft handler of this
    /// query
    async fn register_soft_query_handler<H: QueryHandler + 'static>(handler: H)
    where
        Self: Sized,
    {
        let bus = Busstop::instance();
        if !bus.query_has_handler::<Self>().await {
            bus.register_query::<Self>(handler).await;
        }
    }
}

/// Query Handle Manager
/// Manges the middlewares that will be call before the handler
pub struct QueryHandlerManager {
    name: String,
    middleware: MiddlewareManager<DispatchedQuery, DispatchedQuery>,
}

impl QueryHandlerManager {
    /// Creates a new instance
    pub fn new(handler: impl QueryHandler + 'static) -> Self {
        let handler = Arc::new(Box::new(handler));
        Self {
            name: handler.query_handler_name().to_string(),
            middleware: MiddlewareManager::last(move |dispatched, _| {
                let instance = handler.clone();
                Box::pin(async move { instance.clone().handle_query(dispatched).await })
            }),
        }
    }

    /// Returns the for the Query Handler
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Register the next middleware
    pub fn next<M>(&self, middleware: M) -> &Self
    where
        M: FnMut(DispatchedQuery, NextQueryMiddleware) -> BoxFuture<'static, DispatchedQuery>
            + Send
            + 'static,
    {
        self.middleware.next(middleware);
        self
    }

    /// Handle the specified dispatched query
    pub async fn handle(&self, dispatched: DispatchedQuery) -> DispatchedQuery {
        let mut result = self.middleware.send(dispatched).await;
        result.handled = true;

        result
    }

    /// Same as `handle` but allows you to pass the raw type
    pub async fn handle_query<Q: Send + Sync + 'static>(&self, q: Q) -> DispatchedQuery {
        self.handle(DispatchedQuery::new(
            Box::new(q),
            std::any::type_name::<Q>(),
        ))
        .await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl DispatchableQuery for i32 {}

    #[derive(Default)]
    struct QCommandHandler;

    #[async_trait::async_trait]
    impl QueryHandler for QCommandHandler {
        async fn handle_query(&self, q: DispatchedQuery) -> DispatchedQuery {
            if let Some(v) = q.the_query::<i32>() {
                q.set_value(*v);
            }
            q
        }
    }

    #[tokio::test]
    async fn test_query_handler_manager() {
        let manager = QueryHandlerManager::new(QCommandHandler);

        manager.next(|mut q, n| {
            Box::pin(async move {
                if let Some::<&mut i32>(query) = q.the_query_mut() {
                    *query += 2;
                }
                n.call(q).await
            })
        });

        manager.next(|mut q, n| {
            Box::pin(async move {
                if let Some::<&mut i32>(query) = q.the_query_mut() {
                    *query *= 2
                }
                n.call(q).await
            })
        });

        let ans = manager.handle_query(10).await.take_value::<i32>().unwrap();

        assert_eq!(*ans, 22);
    }
}
