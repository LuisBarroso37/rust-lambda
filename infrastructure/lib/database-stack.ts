import type { StackProps } from "aws-cdk-lib";
import { CfnOutput, RemovalPolicy, Stack } from "aws-cdk-lib";
import { AttributeType, BillingMode, Table } from "aws-cdk-lib/aws-dynamodb";
import type { Construct } from "constructs";

export class DatabaseStack extends Stack {
	constructor(scope: Construct, id: string, properties?: StackProps) {
		super(scope, id, properties);

		const table = new Table(this, "Table", {
			partitionKey: {
				name: "#p",
				type: AttributeType.STRING,
			},
			sortKey: {
				name: "#s",
				type: AttributeType.STRING,
			},
			billingMode: BillingMode.PAY_PER_REQUEST,
			pointInTimeRecovery: true,
			deletionProtection: true,
		});

		table.applyRemovalPolicy(RemovalPolicy.RETAIN_ON_UPDATE_OR_DELETE);

		new CfnOutput(this, "TableName", {
			description: "The name of the DynamoDB table",
			value: table.tableName,
			exportName: "Table-TableName",
		});

		new CfnOutput(this, "TableArn", {
			description: "The ARN of the DynamoDB table",
			value: table.tableArn,
			exportName: "Table-TableArn",
		});
	}
}
