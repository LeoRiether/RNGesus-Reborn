use http::StatusCode;
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use rand::{prelude::*, random, thread_rng};
use serde_json::json;
use std::error::Error;

const BOTMENTION: &'static str = "@therngesusbot";

// Parses and executes the text sent by a user, returning the response RNGesus
// sent from the heavens
fn execute(text: &str) -> Option<String> {
    let command_end_index = text.find(' ').unwrap_or(text.len());
    let (cmd, args) = text.split_at(command_end_index);
    let cmd = cmd.trim_end_matches(BOTMENTION);

    match cmd {
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

fn list(text: &str) -> String {
    let args: Vec<_> = text
        .split(',')
        .map(|arg| arg.trim())
        .filter(|arg| !arg.is_empty())
        .collect();
    if args.is_empty() {
        return "Segmentation Fault".into();
    }

    let i = thread_rng().gen_range(0..args.len());
    let chosen = args[i];

    return chosen.into();
}

fn yesno() -> &'static str {
    const YES: &[&str] = &[
        "Yes",
        "Why not?",
        "Of course",
        "Absolutely",
        "Probably",
        "There's no reason not to",
        "I would think so",
        "Do it now",
        "Go ahead",
        "If you must",
        "Sure, sure",
        "I'm not against it",
        "Yeah!",
        "Hell yeah!",
        "Do it, or else...",
        "I'll be waiting for the results",
    ];
    const NO: &[&str] = &[
        "No",
        "NO",
        "...why would you even do that?",
        "Please do not",
        "No way",
        "Hell no!",
        "Nay",
        "Don't do it, or else...",
        "Absolutely not",
        "Absolutely no way whatsoever",
        "No, no, and no",
        "You shouldn't",
    ];
    const MAYBE: &[&str] = &[
        "Maybe",
        "I'm busy now, try again later",
        "Huh, not sure",
        "Just do whatever, I don't care",
        "Decide it yourself",
        "Who knows",
        "Yes, but maybe not",
        "No, but maybe yes",
        "I'd flip a coin",
        "¯\\_(ツ)_/¯",
        "@deadshrugbot",
        "Are you kidding me?",
    ];

    let mut rng = thread_rng();
    let roll = rng.gen_range(0..=9);
    let mut choose_from = |a: &'static [&'static str]| a[rng.gen_range(0..a.len())];

    match roll {
        0..=3 => choose_from(YES),
        4..=7 => choose_from(NO),
        8..=9 => choose_from(MAYBE),
        _ => "Only after fixing this bug",
    }
}

fn dice(arg: &str) -> String {
    let faces = match arg.trim().parse::<i64>() {
        Ok(x) => x,
        Err(_) => return "???".into(),
    };

    let roll = thread_rng().gen_range(1..=faces);
    format!("Rolled a {}", roll)
}

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    macro_rules! err {
        ($reason:expr) => {
            return Ok(Response::builder()
                .status(StatusCode::OK)
                .body($reason.to_string())
                .expect("Something went wrong"))
            // return Err(NowError::new($reason))
        };
    }

    let body: serde_json::Value = match req.body() {
        now_lambda::Body::Binary(data) => match serde_json::from_slice(data) {
            Ok(body) => body,
            Err(_) => err!("couldn't parse json"),
        },
        _ => err!("Request body is not in binary format"),
    };

    println!("body = {:#?}", body);

    let text = match &body["message"]["text"] {
        serde_json::Value::String(x) => x,
        _ => err!("body.message.text does not exist"),
    };

    let chat_id = match &body["message"]["chat"]["id"] {
        serde_json::Value::Number(x) => x.as_i64().unwrap(),
        _ => err!("body.chat.id does not exist"),
    };

    let response_text = match execute(&text) {
        Some(x) => x,
        None => err!("Couldn't execute command"),
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
