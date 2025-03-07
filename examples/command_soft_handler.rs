use busstop::{CommandHandler, DispatchableCommand, DispatchedCommand};
use tracing::Level;

#[tokio::main]
async fn main() {
    // For logging purposes
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .try_init()
        .expect("could not setup tracing");

    if rand::random() {
        CreateUser::command_handler::<CreateUserHandler>().await;
    }

    // 1. Fallback to this handler...
    CreateUser::soft_command_handler::<SecondCreateUserHandler>().await;

    let cmd = CreateUser {
        email: "hello@example.com".to_string(),
    };

    _ = cmd.dispatch_command().await;
}

#[derive(Debug)]
struct CreateUser {
    pub email: String,
}

impl DispatchableCommand for CreateUser {}

#[derive(Default)]
struct CreateUserHandler;

#[busstop::async_trait]
impl CommandHandler for CreateUserHandler {
    async fn handle_command(&self, dc: busstop::DispatchedCommand) -> DispatchedCommand {
        let command = dc.the_command::<CreateUser>();

        println!(
            "handler one handling create user: {:?}",
            command.unwrap().email
        );

        dc
    }
}

#[derive(Default)]
struct SecondCreateUserHandler;

#[busstop::async_trait]
impl CommandHandler for SecondCreateUserHandler {
    async fn handle_command(&self, dc: busstop::DispatchedCommand) -> DispatchedCommand {
        let command = dc.the_command::<CreateUser>();

        println!(
            "handler two is handling create user: {:?}",
            command.unwrap().email
        );

        dc
    }
}
