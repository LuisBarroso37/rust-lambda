use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use serde_dynamo::{from_item, to_attribute_value};
use shared::{get_required_env_variable, init_tracing};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}

#[tracing::instrument(skip(db_client), level = "info")]
async fn handle_request(db_client: &Client, event: Request) -> Result<Response<Body>, Error> {
    let table_name = get_required_env_variable("TABLE_NAME");

    let path_parameters = event.path_parameters();

    let item_id = match path_parameters.first("id") {
        Some(item_id) => item_id,
        None => {
            let resp = Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body("missing path parameter 'id'".into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

    let item = get_item_by_id(db_client, &table_name, item_id).await?;

    if item.is_none() {
        let resp = Response::builder()
            .status(404)
            .header("content-type", "application/json")
            .body("{}".into())
            .map_err(Box::new)?;

        return Ok(resp);
    }

    let json = serde_json::to_string(&item)?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_tracing();

    let aws_config = shared::get_aws_config().await;
    let client = aws_sdk_dynamodb::Client::new(&aws_config);

    run(service_fn(|event: Request| async {
        handle_request(&client, event).await
    }))
    .await
}

#[tracing::instrument(skip(client, table_name), level = "info")]
pub async fn get_item_by_id(
    client: &Client,
    table_name: &str,
    id: &str,
) -> Result<Option<Item>, Error> {
    let partition_key = format!("u#{item_id}", item_id = id);
    let sort_key = format!("u#{item_id}", item_id = id);

    let partition_key_av = to_attribute_value(partition_key)?;
    let sort_key_av = to_attribute_value(sort_key)?;

    let result = client
        .get_item()
        .table_name(table_name)
        .key("#p", partition_key_av)
        .key("#s", sort_key_av)
        .send()
        .await?;

    match result.item {
        None => return Ok(None),
        Some(item) => {
            let item: Item = from_item(item)?;
            Ok(Some(item))
        }
    }
}
