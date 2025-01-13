import { expect, test } from "@jest/globals";
import { App } from "aws-cdk-lib";
import { Template } from "aws-cdk-lib/assertions";

import { RustLambdaStack } from "../lib/rust-lambda-stack";
import { cleanupUnnecessaryChecksForSnapshot } from "./utils";

test("Access Stack Snapshot Test", () => {
	const app = new App();
	const stack = new RustLambdaStack(app, "RustLambdaStack");
	const template = Template.fromStack(stack);

	expect(cleanupUnnecessaryChecksForSnapshot(template)).toMatchSnapshot();
});
