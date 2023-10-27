mod dispatched_query;
mod query_handler;

pub use dispatched_query::DispatchedQuery;
pub use query_handler::QueryHandler;

use crate::Busstop;

#[async_trait::async_trait]
pub trait DispatchableQuery: Send + Sync {
    async fn dispatch_query(self) -> Option<DispatchedQuery>
    where
        Self: Sized + 'static,
    {
        Busstop::instance().dispatch_query(self).await
    }

    async fn query_handler<H: QueryHandler + Default + 'static>()
    where
        Self: Sized,
    {
        Busstop::instance()
            .register_query::<Self>(H::default())
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

    async fn register_query_handler<H: QueryHandler + 'static>(handler: H)
    where
        Self: Sized,
    {
        Busstop::instance().register_query::<Self>(handler).await;
    }
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
