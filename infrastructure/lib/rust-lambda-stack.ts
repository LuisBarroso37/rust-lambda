import * as path from "path";
import { CfnOutput, Fn, RemovalPolicy, Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";
import { Architecture, FunctionUrlAuthType } from "aws-cdk-lib/aws-lambda";
import { LogGroup, RetentionDays } from "aws-cdk-lib/aws-logs";
import { PolicyStatement } from "aws-cdk-lib/aws-iam";

export class RustLambdaStack extends Stack {
	constructor(scope: Construct, id: string, props?: StackProps) {
		super(scope, id, props);

		const tableArn = Fn.importValue("Table-TableArn");
		const tableName = Fn.importValue("Table-TableName");

		const environment = {
			TABLE_NAME: tableName,
		};

		const dynamodbReadPolicy = new PolicyStatement({
			actions: ["dynamodb:Query", "dynamodb:GetItem", "dynamodb:Scan"],
			resources: [tableArn, `${tableArn}/index/*`],
		});
		const dynamodbReadWritePolicy = new PolicyStatement({
			actions: [
				"dynamodb:Query",
				"dynamodb:GetItem",
				"dynamodb:Scan",
				"dynamodb:PutItem",
				"dynamodb:UpdateItem",
			],
			resources: [tableArn, `${tableArn}/index/*`],
		});

		const getAllFunction = new RustFunction(this, "GetAllFunction", {
			manifestPath: path.join(__dirname, "..", "..", "rust_lambda"),
			binaryName: "get_all",
			runtime: "provided.al2023",
			architecture: Architecture.ARM_64,
			environment,
		});

		getAllFunction.addToRolePolicy(dynamodbReadPolicy);

		new LogGroup(this, "LogGroup1", {
			logGroupName: `/aws/lambda/${getAllFunction.functionName}`,
			retention: RetentionDays.TEN_YEARS,
			removalPolicy: RemovalPolicy.RETAIN,
		});

		const insertFunction = new RustFunction(this, "InsertFunction", {
			manifestPath: path.join(__dirname, "..", "..", "rust_lambda"),
			binaryName: "insert",
			runtime: "provided.al2023",
			architecture: Architecture.ARM_64,
			environment,
		});

		insertFunction.addToRolePolicy(dynamodbReadWritePolicy);

		new LogGroup(this, "LogGroup2", {
			logGroupName: `/aws/lambda/${insertFunction.functionName}`,
			retention: RetentionDays.TEN_YEARS,
			removalPolicy: RemovalPolicy.RETAIN,
		});

		const getAllFunctionUrl = getAllFunction.addFunctionUrl({
			authType: FunctionUrlAuthType.NONE,
		});

		const insertFunctionUrl = insertFunction.addFunctionUrl({
			authType: FunctionUrlAuthType.NONE,
		});

		new CfnOutput(this, "GetAllFunctionUrl", {
			value: getAllFunctionUrl.url,
		});

		new CfnOutput(this, "InsertFunctionUrl", {
			value: insertFunctionUrl.url,
		});
	}
}
