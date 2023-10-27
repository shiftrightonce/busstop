use busstop::{Busstop, CommandHandler};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    // For logging purposes
    SimpleLogger::new().init().unwrap();

    // 1. Get and instance of the bus
    let bus = Busstop::instance();

    // 2. Create and instance the command handler
    let handler = CreateUserHandler;

    // 3. Register the handler
    bus.register_command::<CreateUser>(handler).await;

    // 4. Dispatch command
    bus.dispatch_command(CreateUser {
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
    async fn handle_command(&self, dc: busstop::DispatchedCommand) {
        let command = dc.the_command::<CreateUser>();

        println!(
            "handling \"create user\" command: {:?}",
            command.unwrap().email
        );
    }
}
