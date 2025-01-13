# Infrastructure

## Useful commands

- `npm run build` compile typescript to js
- `npm run test` perform the jest unit tests
- `cdk deploy` deploy this stack to your default AWS account/region
- `cdk diff` compare deployed stack with current state
- `cdk synth` emits the synthesized CloudFormation template

## Deploy stack that creates AWS role that gives temporary credentials to github to be able to deploy cdk stack

AWS_PROFILE=<your-profile> npx cdk deploy GithubActionsRoleStack --parameters RepositoryOwner=<your-org-or-your-profile> --parameters GitHubRepository=<your-repo>
