use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use serde_dynamo::to_attribute_value;
use shared::{get_required_env_variable, init_tracing};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}

/// You can see more examples in Runtime's repository:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
#[tracing::instrument(skip(db_client), level = "info")]
async fn handle_request(db_client: &Client, event: Request) -> Result<Response<Body>, Error> {
    tracing::info!("Received request to add item");
    let table_name = get_required_env_variable("TABLE_NAME");

    let body = event.body();
    let s = std::str::from_utf8(body).expect("invalid utf-8 sequence");

    let item = match serde_json::from_str::<Item>(s) {
        Ok(item) => item,
        Err(err) => {
            let resp = Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(err.to_string().into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

    add_item(db_client, &table_name, item.clone()).await?;

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
pub async fn add_item(client: &Client, table_name: &str, item: Item) -> Result<(), Error> {
    let user_id = Uuid::new_v4();
    let primary_key = format!("u#{user_id}", user_id = user_id);
    let sort_key = format!("u#{user_id}", user_id = user_id);

    tracing::info!("Adding item with {id} to the table", id = user_id);

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

    request.send().await?;

    tracing::info!("Item added successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use lambda_http::http::Request;

    use super::*;

    #[tokio::test]
    async fn it_successfully_adds_item_to_database() {
        let table_name = "DatabaseStack-Table";
        std::env::set_var("TABLE_NAME", table_name);

        let test_dynamodb_config = shared::test::get_dynamodb_test_config().await;
        let dynamodb_client = aws_sdk_dynamodb::Client::new(&test_dynamodb_config);

        shared::test::create_table(&dynamodb_client, table_name)
            .await
            .expect("Table creation should succeed");

        let item = Item {
            username: "test.user".to_string(),
            first_name: "test".to_string(),
            last_name: "user".to_string(),
            age: 30,
        };

        let body = serde_json::to_string(&item).expect("The item should be serializable");

        let request = Request::builder()
            .method("POST")
            .body(Body::Text(body))
            .unwrap();

        let result = handle_request(&dynamodb_client, request)
            .await
            .expect("The request should succeed");

        let s = std::str::from_utf8(result.body()).expect("Invalid utf-8 sequence");

        let response =
            serde_json::from_str::<Item>(s).expect("The response body should be a valid item");

        assert!(result.status().is_success());
        assert_eq!(response, item);

        shared::test::delete_table(&dynamodb_client, table_name)
            .await
            .expect("Table deletion should succeed");
    }
}
