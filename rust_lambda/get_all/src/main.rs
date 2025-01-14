use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use serde_dynamo::aws_sdk_dynamodb_1::from_items;
use shared::{get_required_env_variable, init_tracing};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}

/// You can see more examples in Runtime's repository:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
#[tracing::instrument(skip(db_client))]
async fn handle_request(db_client: &Client) -> Result<Response<Body>, Error> {
    tracing::info!("Received request to get all items");
    let table_name = get_required_env_variable("TABLE_NAME");

    let items = get_all(db_client, &table_name).await?;

    let j = serde_json::to_string(&items)?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(j.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_tracing();

    let aws_config = shared::get_aws_config().await;
    let client = aws_sdk_dynamodb::Client::new(&aws_config);

    run(service_fn(|_event: Request| async {
        handle_request(&client).await
    }))
    .await
}

#[tracing::instrument(skip(client, table_name))]
pub async fn get_all(client: &Client, table_name: &str) -> Result<Vec<Item>, Error> {
    let mut last_evaluated_key = None;
    let mut items_array: Vec<Item> = Vec::new();

    tracing::info!("Getting all items from the table");

    loop {
        let request = client
            .scan()
            .table_name(table_name)
            .set_exclusive_start_key(last_evaluated_key)
            .limit(10);
        let resp = request.send().await?;

        if let Some(items) = resp.items {
            let mut found_items: Vec<Item> = from_items(items)?;
            tracing::info!("Got {:?} items from DynamoDB", found_items.len());

            items_array.append(&mut found_items);
        }

        last_evaluated_key = resp.last_evaluated_key;

        if last_evaluated_key.is_none() {
            break;
        }
    }

    tracing::info!(
        "Found {number_of_items} items in total",
        number_of_items = items_array.len()
    );

    Ok(items_array)
}
