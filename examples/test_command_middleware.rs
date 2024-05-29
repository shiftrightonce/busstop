use busstop::{CommandHandler, CommandHandlerManager, DispatchableCommand, DispatchedCommand};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    // For logging purposes
    SimpleLogger::new().init().unwrap();

    // This example is illustrating how you may test your middlewares for a particular
    // command.

    // 1. Create an instance of the command handler manager
    //    The manager takes an instance of the command handler. Could be useful to test
    //    different handlers for the same command...
    let manager = CommandHandlerManager::new(CreateUserHandler);

    // 2. Add one or more middlewares
    //    Only middleware registered directly on this manager will be
    //    called.
    manager.next(|d, n| {
        Box::pin(async move {
            log::debug!("middleware 1");
            n.call(d).await
        })
    });

    manager.next(|d, n| {
        Box::pin(async move {
            log::debug!("middleware 2");
            n.call(d).await
        })
    });

    // 3. Create an instance of the command
    let cmd = CreateUser {
        email: "james@james.com".to_string(),
    };

    _ = manager.handle_command(cmd).await;
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
