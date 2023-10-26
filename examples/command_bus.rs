use busstop::{Busstop, CommandHandler};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let command_bus = Busstop::instance();

    let handler = CreateUserHandler;
    command_bus.register_command::<CreateUser>(handler).await;

    command_bus
        .dispatch_command(CreateUser {
            email: "hello@world.com".to_string(),
        })
        .await;
}

struct CreateUser {
    pub email: String,
}

struct CreateUserHandler;

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
