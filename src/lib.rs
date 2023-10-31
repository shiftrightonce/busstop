//! Busstop is a command and query bus crate
//!
//! ## Command example
//!
//! ```rust
//!#  #![allow(dead_code)]
//! use busstop::{CommandHandler, DispatchableCommand};
//!
//! #[tokio::main]
//! async fn main() {
//!   CreateUser::command_handler::<CreateUserHandler>().await;
//!   let cmd = CreateUser { email: "foo@bar.com".to_string() };
//!   cmd.dispatch_command().await;
//! }
//!
//! #[derive(Debug)]
//! struct CreateUser {
//!   pub email: String
//! }
//!
//! impl DispatchableCommand for CreateUser{}
//!
//! #[derive(Default)]
//! struct CreateUserHandler;
//!
//! #[busstop::async_trait]
//! impl CommandHandler for CreateUserHandler {
//!    async fn handle_command(&self, dc: busstop::DispatchedCommand) -> busstop::DispatchedCommand {
//!         let command = dc.the_command::<CreateUser>();
//!         println!("User with email'{:?}' was created", &command.unwrap().email);
//!
//!        dc
//!    }
//! }
//!
//! ```
//! ## Query example
//! ```rust
//! use busstop::{DispatchableQuery, DispatchedQuery, QueryHandler};
//! #[tokio::main]
//! async fn main() {
//!   SumOfQuery::query_handler::<HandleSumOfQuery>().await;
//!   let query = SumOfQuery { numbers: vec![6,7,8], };
//!   let result = query.dispatch_query().await;
//!
//!      println!("Ans: {:#?}", result.value::<i32>());
//! }
//!
//! #[derive(Debug)]
//! struct SumOfQuery {
//!    pub numbers: Vec<i32>
//! }
//!
//! impl DispatchableQuery for SumOfQuery{}
//!
//! #[derive(Default)]
//! struct HandleSumOfQuery;
//!
//! #[busstop::async_trait]
//! impl QueryHandler for HandleSumOfQuery {
//!    async fn handle_query(&self, dispatched: busstop::DispatchedQuery) -> DispatchedQuery {
//!      let query = dispatched.the_query::<SumOfQuery>();
//!
//!        let sum = if let Some(subject) = query {
//!            log::info!("summing up: {:?}", subject.numbers);
//!            subject.numbers.iter().fold(0, |sum, n| sum + n)
//!        } else {
//!            0
//!        };
//!
//!        println!("handling 'sum of query'. sum: {:?}", &sum);
//!
//!        dispatched.set_value(sum);
//!
//!        dispatched
//!   }
//! }
//! ```
mod busstop;
mod command;
mod query;

pub use async_trait::async_trait;

pub use busstop::Busstop;

pub use command::CommandHandler;
pub use command::DispatchableCommand;
pub use command::DispatchedCommand;

pub use query::DispatchableQuery;
pub use query::DispatchedQuery;
pub use query::QueryHandler;
