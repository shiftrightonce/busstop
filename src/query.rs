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

    async fn register_query_handler<H: QueryHandler + 'static>(handler: H)
    where
        Self: Sized,
    {
        Busstop::instance().register_query::<Self>(handler).await;
    }
}
