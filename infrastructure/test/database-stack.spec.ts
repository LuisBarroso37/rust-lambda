import { expect, test } from "@jest/globals";
import { App } from "aws-cdk-lib";
import { Template } from "aws-cdk-lib/assertions";

import { DatabaseStack } from "../lib/database-stack";

test("Access Stack Snapshot Test", () => {
	const app = new App();
	const stack = new DatabaseStack(app, "DatabaseStack");
	const template = Template.fromStack(stack);

	expect(template.toJSON()).toMatchSnapshot();
});
