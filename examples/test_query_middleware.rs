use busstop::{DispatchableQuery, DispatchedQuery, QueryHandler, QueryHandlerManager};
use tracing::Level;

#[tokio::main]
async fn main() {
    // For logging purposes
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .try_init()
        .expect("could not setup tracing");

    // We want to test the "add" and "subtract" middlewares that used  on
    // the "MathQueryCommand"

    // 1. Create an instance of the query handler manager.
    //    It takes an instance of the query handler
    let manager = QueryHandlerManager::new(MathQueryCommandHandler).await;

    // 2. Call "next" on the manager to register a middleware,
    //    Register the "addition" middleware
    manager
        .next(|q, n| {
            Box::pin(async move {
                match q.the_query::<MathQueryCommand>() {
                    Some(MathQueryCommand::Add(n1, n2)) => {
                        q.set_value(add(*n1, *n2));
                        q
                    }
                    _ => n.call(q).await,
                }
            })
        })
        .await;

    //   Register the "subtraction" middleware
    manager
        .next(|q, n| {
            Box::pin(async move {
                if let Some(MathQueryCommand::Subtract(n1, n2)) = q.the_query() {
                    q.set_value(subtract(*n1, *n2));
                    q
                } else {
                    n.call(q).await
                }
            })
        })
        .await;

    // test addition
    println!(
        "addition result: {:?}",
        manager
            .handle_query(MathQueryCommand::Add(4, 17))
            .await
            .take_value::<usize>()
    );

    // test subtraction
    println!(
        "subtraction result: {:?}",
        manager
            .handle_query(MathQueryCommand::Subtract(65, 7))
            .await
            .take_value::<usize>()
    );
}

#[allow(dead_code)]
enum MathQueryCommand {
    Add(usize, usize),
    Subtract(usize, usize),
    Multiply(usize, usize),
    Divide(usize, usize),
}

impl DispatchableQuery for MathQueryCommand {}

#[derive(Debug, Default)]
struct MathQueryCommandHandler;

#[busstop::async_trait]
impl QueryHandler for MathQueryCommandHandler {
    async fn handle_query(&self, query: DispatchedQuery) -> DispatchedQuery {
        // .. handler logic
        query
    }
}

fn add(n1: usize, n2: usize) -> usize {
    n1 + n2
}

fn subtract(n1: usize, n2: usize) -> usize {
    n1 - n2
}
