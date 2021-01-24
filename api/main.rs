use http::StatusCode;
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use rand::prelude::*;
use std::error::Error;

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    // let body = match req.body() {
    //     Binary(data) =>
    //     _ => return Err(NowError::new("Request body is not in binary format"));
    // }
    // let body = deserialize(req);
    // println!("body = {:?}", body);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        // .body("user endpoint")
        .body(format!("body = {:?}", req.body()))
        .expect("Internal Server Error");

    Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
