use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use serde_dynamo::to_attribute_value;
use shared::get_dynamodb_client;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}

/// You can see more examples in Runtime's repository:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn handle_request(db_client: &Client, event: Request) -> Result<Response<Body>, Error> {
    let body = event.body();
    let s = std::str::from_utf8(body).expect("invalid utf-8 sequence");

    tracing::info!(payload = %s, "JSON Payload received");

    let item = match serde_json::from_str::<Item>(s) {
        Ok(item) => item,
        Err(err) => {
            let resp = Response::builder()
                .status(400)
                .header("content-type", "text/html")
                .body(err.to_string().into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

    add_item(db_client, item.clone(), "lambda_dyno_example").await?;

    let j = serde_json::to_string(&item)?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(j.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let client = get_dynamodb_client().await;

    run(service_fn(|event: Request| async {
        handle_request(&client, event).await
    }))
    .await
}

pub async fn add_item(client: &Client, item: Item, table_name: &str) -> Result<(), Error> {
    let user_id = Uuid::new_v4();
    let primary_key = format!("u#{user_id}", user_id = user_id.to_string());
    let sort_key = format!("u#{user_id}", user_id = user_id.to_string());

    let primary_key_av = to_attribute_value(primary_key)?;
    let sort_key_av = to_attribute_value(sort_key)?;
    let user_av = to_attribute_value(item.username)?;
    let first_name_av = to_attribute_value(item.first_name)?;
    let last_name_av = to_attribute_value(item.last_name)?;
    let age_av = to_attribute_value(item.age)?;

    let request = client
        .put_item()
        .table_name(table_name)
        .item("#p", primary_key_av)
        .item("#s", sort_key_av)
        .item("username", user_av)
        .item("first_name", first_name_av)
        .item("last_name", last_name_av)
        .item("age", age_av);

    tracing::info!("adding item to DynamoDB");

    request.send().await?;

    Ok(())
}
