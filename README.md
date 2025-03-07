# Busstop

**Busstop is a command and query bus crate**

---

<details>
<summary>
  Command example
</summary>

```rust
use busstop::{CommandHandler, DispatchableCommand};


#[tokio::main]
async fn main() {
    // 1. Register the handler for "CreateUser" command
    CreateUser::command_handler::<CreateUserHandler>().await;

    // 2. Create an instance of the command
    let cmd = CreateUser {
        email: "james@brown.com".to_string(),
    };

    // 3. Dispatch the command
    cmd.dispatch_command().await;
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
    async fn handle_command(&self, dc: busstop::DispatchedCommand) -> busstop::DispatchedCommand {
        let command = dc.the_command::<CreateUser>();

        println!("handling 'create user' : {:?}", command.unwrap().email);

        dc
    }
}

```

</details>
<details>
  <summary>
   Query example
  </summary>

```rust
use busstop::{DispatchableQuery, DispatchedQuery, QueryHandler};

#[tokio::main]
async fn main() {

    // 1. Register query handler
    SumOfQuery::query_handler::<HandlerSumOfQuery>().await;

    // 2. Create an instance of the query
    let query = SumOfQuery {
        numbers: vec![2, 4, 6, 8],
    };

    // 3. Dispatch the query
    let result = query.dispatch_query().await;

    // 4. Use the returned value
    if let Some(d) = result {
        println!("Answer returned: {:#?}", d.value::<i32>());
    }
}

// 5. Create a Query Struct
#[derive(Debug)]
struct SumOfQuery {
    pub numbers: Vec<i32>,
}

// 6. Implement the "DispatchableQuery" trait for "SumOfQuery"
impl DispatchableQuery for SumOfQuery {}

// 7. Create a Handler struct
#[derive(Default)]
struct HandlerSumOfQuery;

// 8. Implement "QueryHandler" trait for "HandleSumOfQuery"
#[busstop::async_trait]
impl QueryHandler for HandlerSumOfQuery {
    async fn handle_query(&self, dispatched_query: busstop::DispatchedQuery) -> DispatchedQuery {
        // 9. Get the query instance
        let query = dispatched_query.the_query::<SumOfQuery>(); 

        let sum = if let Some(subject) = query {
            log::info!("summing up: {:?}", subject.numbers);
            subject.numbers.iter().fold(0, |sum, n| sum + n)
        } else {
            0
        };

        println!("handling 'sum of query'. sum: {:?}", &sum);

        // 10. Make sure to set the value to return
        dispatched_query.set_value(sum);

        dispatched_query
    }
}

```

</details>



## Examples
The [examples](https://github.com/shiftrightonce/busstop/tree/main/examples) folder contains simple and full examples. If none of the examples are helpful,
please reach out with your use case and I  try to provide one.


## Feedback
If you find this crate useful, please star the repository. Submit your issues and recommendations as well.

## License

### The MIT License (MIT)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.