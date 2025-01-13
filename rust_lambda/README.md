## Install Rust

curl https://sh.rustup.rs -sSf | sh

## Install cargo lambda (used to test, build and deploy AWS lambdas)

brew tap cargo-lambda/cargo-lambda
brew install cargo-lambda

## Start a new project

cargo lambda new rust_lambda

## Run lambda locally for testing

cargo lambda watch

## Locally test the function

cargo lambda invoke --data-ascii "{ \"command\": \"hi\" }"

If function integrates with api gateway then we can use this command:

cargo lambda invoke http-lambda --data-example apigw-request

## Url used to check local lambda that is currently running

http://localhost:9000/lambda-url/rust_lambda

## Build lambda function

cargo lambda build --release

Note: Use flag --arm64 if you want to use Graviton processors on AWS Lambda

## Deploy lambda function

cargo lambda deploy
