use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_sdk_dynamodb::{
    operation::{create_table::CreateTableOutput, delete_table::DeleteTableOutput},
    types::{
        AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
    },
    Client, Error,
};

pub async fn get_dynamodb_test_config() -> SdkConfig {
    aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new("eu-west-1"))
        .test_credentials()
        .endpoint_url("http://localhost:8000")
        .load()
        .await
}

pub async fn create_table(client: &Client, table_name: &str) -> Result<CreateTableOutput, Error> {
    let partition_key_attribute_definition = AttributeDefinition::builder()
        .attribute_name("#p")
        .attribute_type(ScalarAttributeType::S)
        .build()?;

    let sort_key_attribute_definition = AttributeDefinition::builder()
        .attribute_name("#s")
        .attribute_type(ScalarAttributeType::S)
        .build()?;

    let partition_key_schema = KeySchemaElement::builder()
        .attribute_name("#p")
        .key_type(KeyType::Hash)
        .build()?;

    let sort_key_schema = KeySchemaElement::builder()
        .attribute_name("#s")
        .key_type(KeyType::Range)
        .build()?;

    let provisioned_throughput = ProvisionedThroughput::builder()
        .read_capacity_units(1)
        .write_capacity_units(1)
        .build()?;

    client
        .create_table()
        .table_name(table_name)
        .key_schema(partition_key_schema)
        .key_schema(sort_key_schema)
        .attribute_definitions(partition_key_attribute_definition)
        .attribute_definitions(sort_key_attribute_definition)
        .provisioned_throughput(provisioned_throughput)
        .send()
        .await
        .map_err(Error::from)
}

pub async fn delete_table(client: &Client, table_name: &str) -> Result<DeleteTableOutput, Error> {
    client
        .delete_table()
        .table_name(table_name)
        .send()
        .await
        .map_err(Error::from)
}
