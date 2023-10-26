use super::DispatchedQuery;

#[async_trait::async_trait]
pub trait QueryHandler: Send + Sync {
    async fn handle_query(&self, command: DispatchedQuery) -> DispatchedQuery;

    fn query_handler_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}
