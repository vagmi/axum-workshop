# Axum Workshop

This the code associated with the workshop for That Conference, WI 2023.

## Step 0

This contains the basic hello world code that verifies if the toolchain and the
editor is setup correctly.

## Step 1

Lets write a module with a unit test and call it from the module that has the
main function.

## Step 2

Add tokio and we discuss how the runtime is distinct from the language even
though the language has support for async await. We'll expand the `tokio::main`
macro to see how the task is setup.

## Step 3

We now add Axum and start a http server. We use path parameters and see how
the Axum extractors work. We also see how we can test handlers independently 
and how to test handlers and the router.

## Step 4

Step 4 adds state and demonstrates how we can nest routers for API. We also
introduce `debug_handler` for better error messages during development.

## Step 5

Step 5 adds SQLX with migrations a single model and unit tests for testing DAOs.
We see how we can use the State extractor to pass in a database connection pool
to use it in our extractors.

## Step 6

Step 6 adds error handling to eliminate `unwrap()` calls from our handlers and 
have our handlers be fallible.

## Step 7

We write a custom extractor to process the JWT token from the header and 
accessing it from the handler. We also setup logging with tracing_subscriber
and how we can add a trace layer middleware.

