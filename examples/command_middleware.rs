use busstop::{CommandHandler, DispatchableCommand, DispatchedCommand};
use tracing::Level;

#[tokio::main]
async fn main() {
    // For logging purposes
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .try_init()
        .expect("could not setup tracing");

    // 1. Registering a middleware
    //    Middlewares can be registered before or after a command handler is registered.
    //    However, the middleware(s) will not run if you do not have a command handler
    //    for the specified command.
    CreateUser::command_middleware(|p, n| {
        Box::pin(async move {
            tracing::info!(target: "middleware", "|----> middleware 1 was called");
            n.call(p).await // calls the next middleware in the chain
        })
    })
    .await;

    // 2. A second middleware is added. This one uses a function
    CreateUser::command_middleware(|mut p, n| {
        Box::pin(async move {
            tracing::info!(target: "middleware", "|----> middleware 2 was called");
            if let Some(user) = p.the_command_mut::<CreateUser>() {
                tracing::warn!(target: "middleware 2", "New user email: {}", &user.email);
                user.email = sanitize_email(&user.email);
            }
            n.call(p).await
        })
    })
    .await;

    // 3. Register the handler for "CreateUser" command
    CreateUser::command_handler::<CreateUserHandler>().await;

    // 4. Create an instance of the command
    let cmd = CreateUser {
        email: "JaMes Brown@jAmes.c o m".to_string(),
    };

    // 5. Dispatch the command
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

// Fake email sanitization function
fn sanitize_email(email: &str) -> String {
    email.split(' ').collect::<String>().to_ascii_lowercase()
}
