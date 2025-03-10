use busstop::{CommandHandler, DispatchableCommand, DispatchedCommand};
use tracing::Level;

#[tokio::main]
async fn main() {
    // For logging purposes
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .try_init()
        .expect("could not setup tracing");

    // 1. Register the handler for "CreateUser" command
    CreateUser::command_handler::<CreateUserHandler>().await;

    // 2. Create an instance of the command
    let cmd = CreateUser {
        email: "james@james.com".to_string(),
    };

    // 3. Dispatch the command
    _ = cmd.dispatch_command().await;
}

// 4. Create the command struct
#[derive(Debug)]
struct CreateUser {
    pub email: String,
}

// 5. Make the Command dispatchable (see step 3)
impl DispatchableCommand for CreateUser {}

// 6. Create the handler struct
#[derive(Default)]
struct CreateUserHandler;

// 7. Implement the "CommandHandler" trait for this handler
#[busstop::async_trait]
impl CommandHandler for CreateUserHandler {
    async fn handle_command(&self, dc: busstop::DispatchedCommand) -> DispatchedCommand {
        // 8. Get the "CreateUser" command instance
        let command = dc.the_command::<CreateUser>();

        println!("handling create user: {:?}", command.unwrap().email);

        dc
    }
}
