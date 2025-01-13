#!/usr/bin/env node
import "source-map-support/register";
import { App } from "aws-cdk-lib";

import { RustLambdaStack } from "../lib/rust-lambda-stack";
import { DatabaseStack } from "../lib/database-stack";

const app = new App();

new RustLambdaStack(app, "RustLambdaStack", {});
new DatabaseStack(app, "DatabaseStack", {});
