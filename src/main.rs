use std::error::Error;

use lambda_http::{lambda, Request, Response, Body};
use lambda_runtime::{error::HandlerError, Context};
use log::{self, info};
use simple_logger;

mod foods;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Debug,Deserialize,Default)]
struct Req {
  #[serde(default)]
  queryResult: QueryResult
}

#[derive(Debug,Deserialize,Default)]
struct QueryResult {
  #[serde(default)]
  queryText: String,
  #[serde(default)]
  parameters: Parameters
}

#[derive(Debug,Deserialize,Default)]
struct Parameters {
  #[serde(default)]
  food: String
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: Request, c: Context) -> Result<Response<Body>, HandlerError> {
  let query: Req = serde_json::from_slice(e.body().as_ref()).unwrap();
  info!("{:#?}", query);
  let food: String = query.queryResult.parameters.food;
  info!("{}", foods::foods::get(&food));
  Ok(
    Response::builder()
     .status(200)
     .header("Content-Type", "application/json; charset=UTF-8")
     .body(
       format!(
        r#"
          {{
            "fulfillmentText": "{}"
          }}"#, 
        foods::foods::get(&food)
        ).into(),
      )
     .expect("none")
  )
}
