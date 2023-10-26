use busstop::{DispatchableQuery, DispatchedQuery, QueryHandler};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    NameQuery::query_handler::<NameQueryHandler>().await;

    let name = NameQuery {
        email: "hello@hello.com".to_string(),
    };
    let result = name.dispatch_query().await;

    if let Some(d) = result {
        let ans = d.value::<Vec<usize>>();
        println!("value returned: {:#?}", ans.unwrap());
    }
}

#[derive(Debug)]
struct NameQuery {
    pub email: String,
}

impl DispatchableQuery for NameQuery {}

#[derive(Default)]
struct NameQueryHandler;

#[busstop::async_trait]
impl QueryHandler for NameQueryHandler {
    async fn handle_query(&self, dq: busstop::DispatchedQuery) -> DispatchedQuery {
        let query = dq.the_query::<NameQuery>();
        let mut v = Vec::new();

        v.push(1);
        v.push(2);
        v.push(query.unwrap().email.len());

        dq.set_value(v);

        println!("handling name query : {:?}", query.unwrap().email);

        dq
    }
}
