use super::DispatchedQuery;

/// A query's handler must implement this trait
#[async_trait::async_trait]
pub trait QueryHandler: Send + Sync {
    /// This method is call to handle the dispatched query
    async fn handle_query(&self, dispatched: DispatchedQuery) -> DispatchedQuery;

    /// A unique name for this handler
    /// By default, the path to the type is used
    fn query_handler_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}
