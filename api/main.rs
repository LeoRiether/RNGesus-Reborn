use http::StatusCode;
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use rand::prelude::*;
use serde_json::json;
use std::error::Error;

// Parses and executes the text sent by a user, returning the response RNGesus
// sent from the heavens
fn execute(text: &str) -> String {
    "Your bot code works perfectly".into()
}

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    let body: serde_json::Value = match req.body() {
        now_lambda::Body::Binary(data) => serde_json::from_slice(data).expect("Incorrectly formatted json"),
        _ => return Err(NowError::new("Request body is not in binary format"))
    };

    let text = match &body["message"]["text"] {
        serde_json::Value::String(x) => x,
        _ => return Err(NowError::new("body.message.text does not exist")),
    };

    let chat_id = match &body["chad"]["id"] {
        serde_json::Value::Number(x) => x,
        _ => return Err(NowError::new("body.chat.id does not exist")),
    };

    let response_json = json!({
        "method": "sendMessage",
        "chat_id": chat_id,
        "text": execute(&text),
    });

    println!("response = {:#?}", response_json);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(response_json.to_string())
        .expect("Internal Server Error");

    Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
