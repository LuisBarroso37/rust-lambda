/**
 * @type {import('eslint').Linter.Config}
 */
module.exports = {
	ignorePatterns: [
		".eslintrc.js",
		".eslintrc.cjs",
		"jest.config.js",
		"dist/",
		"__bundles__/",
		"cdk.out/",
		"jest.setup.cjs",
	],
	extends: [
		"eslint:recommended",
		"plugin:@typescript-eslint/strict-type-checked",
		"plugin:@typescript-eslint/stylistic-type-checked",
		"plugin:prettier/recommended",
	],
	parser: "@typescript-eslint/parser",
	plugins: ["@typescript-eslint", "jest"],
	root: true,
	settings: {
		"import/resolver": {
			node: {
				extensions: [".js", ".ts", ".json"],
			},
			typescript: true,
		},
	},
	parserOptions: {
		tsconfigRootDir: __dirname,
		project: ["./tsconfig.json"],
	}
};
