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
//!            tracing::info!("summing up: {:?}", subject.numbers);
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

pub use command::*;
pub use query::*;

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_command_without_handler() {
        struct FooCommand;
        #[async_trait::async_trait]
        impl DispatchableCommand for FooCommand {}

        let handled = FooCommand.dispatch_command().await;
        assert_eq!(handled, false);
    }

    #[tokio::test]
    async fn test_command_with_handler() {
        struct FooCommand;
        #[async_trait::async_trait]
        impl DispatchableCommand for FooCommand {}

        struct FooCommandHandler;
        #[async_trait::async_trait]
        impl CommandHandler for FooCommandHandler {
            async fn handle_command(&self, dispatched: DispatchedCommand) -> DispatchedCommand {
                dispatched
            }
        }

        FooCommand::register_command_handler(FooCommandHandler).await;

        let handled = FooCommand.dispatch_command().await;
        assert_eq!(handled, true);
    }

    #[tokio::test]
    async fn test_get_command_ref() {
        struct BroadcastCommand {
            message: String,
        }
        #[async_trait::async_trait]
        impl DispatchableCommand for BroadcastCommand {}

        #[derive(Debug, Default)]
        struct BroadcastCommandHandler;
        #[async_trait::async_trait]
        impl CommandHandler for BroadcastCommandHandler {
            async fn handle_command(&self, dispatched: DispatchedCommand) -> DispatchedCommand {
                let command = dispatched.the_command::<BroadcastCommand>();

                assert_eq!(
                    command.is_some(),
                    true,
                    "Could not get the dispatched command"
                );
                assert_eq!(command.as_ref().unwrap().message, "--test--");
                dispatched
            }
        }

        BroadcastCommand::command_handler::<BroadcastCommandHandler>().await;
        BroadcastCommand {
            message: String::from("--test--"),
        }
        .dispatch_command()
        .await;
    }

    #[tokio::test]
    async fn test_get_command_mut_ref() {
        struct BroadcastCommand {
            message: String,
            attempts: i32,
        }
        #[async_trait::async_trait]
        impl DispatchableCommand for BroadcastCommand {}

        #[derive(Debug, Default)]
        struct BroadcastCommandHandler;
        #[async_trait::async_trait]
        impl CommandHandler for BroadcastCommandHandler {
            async fn handle_command(&self, mut dispatched: DispatchedCommand) -> DispatchedCommand {
                let command = dispatched.the_command_mut::<BroadcastCommand>();

                assert_eq!(
                    command.is_some(),
                    true,
                    "Could not get the dispatched command"
                );
                assert_eq!(command.as_ref().unwrap().message, "--test--");

                if let Some(inner) = command {
                    inner.attempts = 10;
                    assert_eq!(inner.attempts, 10);
                }

                dispatched
            }
        }

        BroadcastCommand::command_handler::<BroadcastCommandHandler>().await;
        BroadcastCommand {
            message: String::from("--test--"),
            attempts: 0,
        }
        .dispatch_command()
        .await;
    }

    #[tokio::test]
    async fn test_take_command() {
        struct BroadcastCommand {
            message: String,
            attempts: i32,
        }
        #[async_trait::async_trait]
        impl DispatchableCommand for BroadcastCommand {}

        #[derive(Debug, Default)]
        struct BroadcastCommandHandler;
        #[async_trait::async_trait]
        impl CommandHandler for BroadcastCommandHandler {
            async fn handle_command(&self, mut dispatched: DispatchedCommand) -> DispatchedCommand {
                let command = dispatched.take_command::<BroadcastCommand>();

                assert_eq!(
                    command.is_some(),
                    true,
                    "Could not get the dispatched command"
                );
                assert_eq!(command.as_ref().unwrap().message, "--test--");

                if let Some(mut inner) = command {
                    inner.attempts = 10;
                    assert_eq!(inner.attempts, 10);
                }

                let command = dispatched.take_command::<BroadcastCommand>();
                assert_eq!(command.is_none(), true, "Expect none");

                dispatched
            }
        }

        BroadcastCommand::command_handler::<BroadcastCommandHandler>().await;
        BroadcastCommand {
            message: String::from("--test--"),
            attempts: 0,
        }
        .dispatch_command()
        .await;
    }

    #[tokio::test]
    async fn test_query_without_handler() {
        struct NextIdQuery;
        #[async_trait::async_trait]
        impl DispatchableQuery for NextIdQuery {}

        let dispatched = NextIdQuery.dispatch_query().await;

        assert_eq!(dispatched.handled(), false);
    }

    #[tokio::test]
    async fn test_query_handler() {
        struct NextIdQuery;
        #[async_trait::async_trait]
        impl DispatchableQuery for NextIdQuery {}

        struct NextIdQueryHandler;
        #[async_trait::async_trait]
        impl QueryHandler for NextIdQueryHandler {
            async fn handle_query(&self, dispatched: DispatchedQuery) -> DispatchedQuery {
                dispatched
            }
        }

        NextIdQuery::register_query_handler(NextIdQueryHandler).await;

        let dispatched = NextIdQuery.dispatch_query().await;

        assert_eq!(dispatched.handled(), true);
    }
}
