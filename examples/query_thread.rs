use busstop::DispatchableQuery;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    // For logging purposes
    SimpleLogger::new().init().unwrap();

    // 1. Register query handler
    //   Advisable to do this in your main thread
    TheSmallestQuery::query_handler::<HandleTheSmallestQuery>().await;

    // 2. Spawn thread one and dispatch a query
    let handler2 = std::thread::spawn(|| async {
        let query = TheSmallestQuery(500, 500);

        let result = query.dispatch_query().await;

        println!("ans1: {:?}", result.value::<String>());
    });

    // 3. Spawn thread two and dispatch a query
    let handler1 = std::thread::spawn(|| async {
        let query = TheSmallestQuery(500, 600);

        let result = query.dispatch_query().await;

        println!("ans2: {:?}", result.value::<String>());
    });

    // join threads
    _ = handler1.join().unwrap().await;
    _ = handler2.join().unwrap().await;
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

        dispatched.set_value(ans);

        dispatched
    }
}
