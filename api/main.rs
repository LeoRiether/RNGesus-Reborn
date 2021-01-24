use http::StatusCode;
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use rand::prelude::*;
use std::error::Error;

fn deserialize<T>(req: Request<Vec<u8>>) -> serde_json::Result<Request<T>>
    where for<'de> T: serde::de::Deserialize<'de>,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::from_slice(&body)?;
    Ok(Request::from_parts(parts, body))
}

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    let body = deserialize(req);
    println!("body = {:?}", body);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body("user endpoint")
        .expect("Internal Server Error");

    Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
