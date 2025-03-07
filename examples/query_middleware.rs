use busstop::{DispatchableQuery, DispatchedQuery, QueryHandler};
use tracing::Level;

#[tokio::main]
async fn main() {
    // For logging purposes
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .try_init()
        .expect("could not setup tracing");

    // 1. Like command middleware, we can add query middleware before or after the
    //    query handler is register. The middlewares are only executed when there is a
    //    handler.
    //
    //   This middleware is providing an implementation for addition. The query handler
    //  is yet to handle addition. See further down
    MathQueryCommand::query_middleware(|q, n| {
        Box::pin(async move {
            match q.the_query::<MathQueryCommand>() {
                Some(MathQueryCommand::Add(n1, n2)) => {
                    q.set_value(add(*n1, *n2));
                    q
                }
                _ => n.call(q).await, // Pass other operations to the next middleware
            }
        })
    })
    .await;

    // 2. This middleware handle subtraction
    MathQueryCommand::query_middleware(|q, n| {
        Box::pin(async move {
            match q.the_query::<MathQueryCommand>() {
                Some(MathQueryCommand::Subtract(n1, n2)) => {
                    q.set_value(subtract(*n1, *n2));
                    q
                }
                _ => n.call(q).await,
            }
        })
    })
    .await;

    // 3. This middleware is pretending to fake a division by zero bug
    MathQueryCommand::query_middleware(|q, n| {
        Box::pin(async move {
            match q.the_query::<MathQueryCommand>() {
                Some(MathQueryCommand::Divide(n1, n2)) if *n2 == 0 => {
                    q.set_value(divide(*n1, 1));
                    q
                }
                _ => n.call(q).await,
            }
        })
    })
    .await;

    // 4. The actual query handler is being registered
    MathQueryCommand::query_handler::<MathQueryCommandHandler>().await;

    // -- Below are various query calls

    // Divide by zero
    println!(
        "Response for: 600 ➗ 0 = {:?}",
        MathQueryCommand::Divide(600, 0)
            .dispatch_query()
            .await
            .take_value::<usize>()
    );

    // Divide
    println!(
        "Response for: 8701 ➗ 6 = {:?}",
        MathQueryCommand::Divide(8701, 6)
            .dispatch_query()
            .await
            .take_value::<usize>()
    );

    // Add
    println!(
        "Response for: 2 + 2 = {:?}",
        MathQueryCommand::Add(2, 2)
            .dispatch_query()
            .await
            .take_value::<usize>()
    );
    // Subtract
    println!(
        "Response for: 88 - 12 = {:?}",
        MathQueryCommand::Subtract(88, 12)
            .dispatch_query()
            .await
            .take_value::<usize>()
    );

    // Multiply
    println!(
        "Response for: 45 x 8 = {:?}",
        MathQueryCommand::Multiply(45, 8)
            .dispatch_query()
            .await
            .take_value::<usize>()
    );
}

enum MathQueryCommand {
    Add(usize, usize),
    Subtract(usize, usize),
    Multiply(usize, usize),
    Divide(usize, usize),
}

impl DispatchableQuery for MathQueryCommand {}

fn add(n1: usize, n2: usize) -> usize {
    n1 + n2
}

fn subtract(n1: usize, n2: usize) -> usize {
    n1 - n2
}

fn multiple(n1: usize, n2: usize) -> usize {
    n1 * n2
}

fn divide(n1: usize, n2: usize) -> usize {
    n1 / n2
}

#[derive(Debug, Default)]
struct MathQueryCommandHandler;

#[busstop::async_trait]
impl QueryHandler for MathQueryCommandHandler {
    async fn handle_query(&self, mut query: DispatchedQuery) -> DispatchedQuery {
        if let Some(q) = query.take_query::<MathQueryCommand>() {
            match *q {
                MathQueryCommand::Divide(n1, n2) => query.set_value(divide(n1, n2)),
                MathQueryCommand::Multiply(n1, n2) => query.set_value(multiple(n1, n2)),
                _ => unimplemented!("yet to be implemented"),
            }
        }

        query
    }
}
