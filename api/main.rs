use http::StatusCode;
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use rand::{prelude::*, random, thread_rng};
use serde_json::json;
use std::error::Error;

// Parses and executes the text sent by a user, returning the response RNGesus
// sent from the heavens
fn execute(text: &str) -> Option<String> {
    let command_end_index = text.find(' ').unwrap_or(text.len());
    let (cmd, args) = text.split_at(command_end_index);

    match &text[0..command_end_index] {
        "/coin" => Some(coin().into()),
        "/list" => Some(list(args)),
        "/yesno" => Some(yesno().into()),
        "/dice" => Some(dice(args)),

        _ => None,
    }
}

fn coin() -> &'static str {
    match random::<bool>() {
        true => "Heads",
        false => "Tails",
    }
}

fn list(args: &str) -> String {
    "Not implemented yet".into()
}

fn yesno() -> &'static str {
    "Not implemented yet"
}

fn dice(args: &str) -> String {
    "Not implemented yet".into()
}

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    let body: serde_json::Value = match req.body() {
        now_lambda::Body::Binary(data) => {
            serde_json::from_slice(data).expect("Incorrectly formatted json")
        }
        _ => return Err(NowError::new("Request body is not in binary format")),
    };

    println!("body = {:#?}", body);

    let text = match &body["message"]["text"] {
        serde_json::Value::String(x) => x,
        _ => return Err(NowError::new("body.message.text does not exist")),
    };

    let chat_id = match &body["message"]["chat"]["id"] {
        serde_json::Value::Number(x) => x.as_i64().unwrap(),
        _ => return Err(NowError::new("body.chat.id does not exist")),
    };

    let response_text = match execute(&text) {
        Some(x) => x,
        None => return Err(NowError::new("Couldn't execute command")),
    };

    let response_json = json!({
        "method": "sendMessage",
        "chat_id": chat_id,
        "text": response_text,
    });

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
