use busstop::{Busstop, CommandHandler, DispatchedCommand};
use tracing::Level;

#[tokio::main]
async fn main() {
    // For logging purposes
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .try_init()
        .expect("could not setup tracing");

    // 1. Get and instance of the bus
    let bus = Busstop::instance();

    // 2. Create and instance the command handler
    let handler = CreateUserHandler;

    // 3. Register the handler
    bus.register_command::<CreateUser>(handler).await;

    // 4. Dispatch command
    _ = bus
        .dispatch_command(CreateUser {
            email: "hello@world.com".to_string(),
        })
        .await;
}

// 5. Create Command
#[derive(Debug)]
struct CreateUser {
    pub email: String,
}

// 6. Create a handler
struct CreateUserHandler;

// 7. Implement "CommandHandler" for the handler
#[busstop::async_trait]
impl CommandHandler for CreateUserHandler {
    async fn handle_command(&self, dc: busstop::DispatchedCommand) -> DispatchedCommand {
        let command = dc.the_command::<CreateUser>();

        println!(
            "handling \"create user\" command: {:?}",
            command.unwrap().email
        );

        dc
    }
}
