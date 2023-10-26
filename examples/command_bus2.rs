use busstop::{CommandHandler, DispatchableCommand};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    CreateUser::command_handler::<CreateUserHandler>().await;

    let cmd = CreateUser {
        email: "Hello".to_string(),
    };

    cmd.dispatch_command().await;

    println!("command bus");
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
    async fn handle_command(&self, dc: busstop::DispatchedCommand) {
        let command = dc.the_command::<CreateUser>();

        println!("handling create user: {:?}", command.unwrap().email);
    }
}
