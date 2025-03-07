use busstop::DispatchableQuery;
use tracing::Level;

#[tokio::main]
async fn main() {
    // For logging purposes
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .try_init()
        .expect("could not setup tracing");

    // 1. Randomly register the main handler for the query
    if rand::random() {
        TheSmallestQuery::query_handler::<HandleTheSmallestQuery>().await;
    }

    // 2. Fallback to this handler if there is no handler for the query
    TheSmallestQuery::soft_query_handler::<SecondHandleTheSmallestQuery>().await;

    // 3. Create an instance of the query and dispatch it
    let query = TheSmallestQuery(500, 500);
    let result = query.dispatch_query().await;

    println!("ans1: {:?}", result.value::<String>());
}

#[derive(Debug)]
struct TheSmallestQuery(i32, i32);

impl busstop::DispatchableQuery for TheSmallestQuery {}

#[derive(Debug, Default)]
struct HandleTheSmallestQuery;

#[busstop::async_trait]
impl busstop::QueryHandler for HandleTheSmallestQuery {
    async fn handle_query(&self, dispatched: busstop::DispatchedQuery) -> busstop::DispatchedQuery {
        let query = dispatched.the_query::<TheSmallestQuery>();
        let ans = if let Some(q) = query {
            if q.0 < q.1 {
                "left".to_string()
            } else if q.1 < q.0 {
                "right".to_string()
            } else {
                "equal".to_string()
            }
        } else {
            "unknown".to_string()
        };

        dispatched.set_value(format!("first handler ans is: {}", &ans));

        dispatched
    }
}

#[derive(Debug, Default)]
struct SecondHandleTheSmallestQuery;

#[busstop::async_trait]
impl busstop::QueryHandler for SecondHandleTheSmallestQuery {
    async fn handle_query(&self, dispatched: busstop::DispatchedQuery) -> busstop::DispatchedQuery {
        let query = dispatched.the_query::<TheSmallestQuery>();
        let ans = if let Some(q) = query {
            if q.0 < q.1 {
                "left".to_string()
            } else if q.1 < q.0 {
                "right".to_string()
            } else {
                "equal".to_string()
            }
        } else {
            "unknown".to_string()
        };

        dispatched.set_value(format!("second handler ans is: {}", &ans));

        dispatched
    }
}
