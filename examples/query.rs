use busstop::{DispatchableQuery, DispatchedQuery, QueryHandler};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    // For logging purposes
    SimpleLogger::new().init().unwrap();

    // 1. Register query handler
    SumOfQuery::query_handler::<HandleSumOfQuery>().await;

    // 2. Create an instance of the query
    let query = SumOfQuery {
        numbers: vec![2, 4, 6, 8],
    };

    // 3. Dispatch the query
    let result = query.dispatch_query().await;

    // 4. Use the returned value
    if let Some(d) = result {
        println!("Answer returned: {:#?}", d.value::<i32>());
    }
}

// 5. Create a Query Struct
#[derive(Debug)]
struct SumOfQuery {
    pub numbers: Vec<i32>,
}

// 6. Implement the "DispatchableQuery" trait for "SumOfQuery"
impl DispatchableQuery for SumOfQuery {}

// 7. Create a Handler struct
#[derive(Default)]
struct HandleSumOfQuery;

// 8. Implement "QueryHandler" trait for "HandleSumOfQuery"
#[busstop::async_trait]
impl QueryHandler for HandleSumOfQuery {
    async fn handle_query(&self, dq: busstop::DispatchedQuery) -> DispatchedQuery {
        // 9. Get the "SumOfQuery" instance
        let query = dq.the_query::<SumOfQuery>();

        let sum = if let Some(subject) = query {
            log::info!("summing up: {:?}", subject.numbers);
            subject.numbers.iter().fold(0, |sum, n| sum + n)
        } else {
            0
        };

        println!("handling 'sum of query'. sum: {:?}", &sum);

        // 10. Make sure to set the value to return
        dq.set_value(sum);

        dq
    }
}
