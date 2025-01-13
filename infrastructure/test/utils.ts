import type { Template } from "aws-cdk-lib/assertions";

/**
 * Removes the hashes from the lambdas in the template otherwise we always need to update the snapshot when assets change as the S3Key will change
 * Removes the hashes from the versions in the resource names
 * Removes the templateUrl from the nested stack
 * @returns JSON That should be matched against the snapshot
 */
export const cleanupUnnecessaryChecksForSnapshot = (template: Template) => {
	const lambdas = template.findResources("AWS::Lambda::Function");
	let json = template.toJSON();
	for (const [lambda] of Object.entries(lambdas)) {
		// eslint-disable-next-line @typescript-eslint/no-unsafe-member-access -- We know that the resource exists, check cloudformation structure
		json.Resources[lambda].Properties.Code.S3Key = "PLACEHOLDER";
	}

	const versions = template.findResources("AWS::Lambda::Version");
	for (const [version] of Object.entries(versions)) {
		const newVersion = (version.split("Version")[0] ?? "") + "Version";
		const jsonString = JSON.stringify(json).replaceAll(version, newVersion);
		json = JSON.parse(jsonString) as Record<string, never>;
	}

	const nestedStack = template.findResources("AWS::CloudFormation::Stack");
	for (const [stack] of Object.entries(nestedStack)) {
		// eslint-disable-next-line @typescript-eslint/no-unsafe-member-access -- We know that the resource exists, check cloudformation structure
		json.Resources[stack].Properties.TemplateURL = "S3URLPLACEHOLDER";
	}

	return json;
};
