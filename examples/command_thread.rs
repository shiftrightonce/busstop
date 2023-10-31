use busstop::{CommandHandler, DispatchableCommand, DispatchedCommand};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    // For logging purposes
    SimpleLogger::new().init().unwrap();

    // 1. Register command handler
    //    In this case for the command "CreateUser"
    CreateUser::command_handler::<CreateUserHandler>().await;

    // 2. We are dispatching the command from another thread
    let t2 = std::thread::spawn(|| async {
        let cmd = CreateUser {
            email: "example@example.com".to_string(),
        };

        _ = cmd.dispatch_command().await;
    });

    _ = t2.join().unwrap().await;
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

        println!("handling create user: {:?}", command.unwrap().email);

        dc
    }
}
