use http::StatusCode;
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use rand::{prelude::*, random, thread_rng};
use serde_json::json;
use std::error::Error;

fn choose_from<T: Copy>(a: &[T]) -> T {
    let i = thread_rng().gen_range(0..a.len());
    a[i]
}

const BOTMENTION: &'static str = "@therngesusbot";

enum BotResponse {
    Message(String),
    LeaveChat,
    DeleteMessage,
    DeleteAndSend(String), // delete the command and send some message
}

// Parses and executes the text sent by a user, returning the response RNGesus
// sent from the heavens
fn execute(text: &str) -> Option<BotResponse> {
    let command_end_index = text.find(' ').unwrap_or(text.len());
    let (cmd, args) = text.split_at(command_end_index);
    let cmd = cmd.trim_end_matches(BOTMENTION);

    use BotResponse::*;
    macro_rules! wrap {
        ($response:expr, $tag:ident) => {
            Some($tag($response.into()))
        };
    }

    match cmd {
        "/coin" => wrap!(coin(), Message),
        "/list" => wrap!(list(args), Message),
        "/yesno" => wrap!(yesno(), Message),
        "/decide" => wrap!(decide(), Message),
        "/dice" => wrap!(dice(args), Message),
        "/rps" => wrap!(rps(), Message),
        "/rpsls" => wrap!(rpsls(), Message),
        "/say" => wrap!(args.trim(), DeleteAndSend),

        "/deicide" => Some(LeaveChat),
        "/deletethis" | "/wakeup" => Some(DeleteMessage),

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
    const TEMPLATES: &[&str] = &[
        "{}, clearly",
        "I choose {}",
        "Has to be {}",
        "{}, isn't it?",
        "It's {}",
        "{} is the chosen one",
        "Couldn't not be {}",
        "I declare {} to be victorious",
        "{}, or suffer the consequences",
        "Either {} or {}",
        "It's {} or nothing",
        "{} without a doubt",
    ];

    let split_char = if text.find(',').is_some() { ',' } else { ' ' };
    let args: Vec<_> = text
        .split(split_char)
        .map(|arg| arg.trim())
        .filter(|arg| !arg.is_empty())
        .collect();
    if args.is_empty() {
        return "Segmentation Fault".into();
    }

    let chosen = choose_from(&args);
    let template = choose_from(TEMPLATES);
    return template.replace("{}", chosen);
}

fn yesno() -> &'static str {
    const YES: &[&str] = &[
        "Yes",
        "Why not?",
        "Of course",
        "Absolutely",
        "Probably",
        "I would think so",
        "Sure, sure",
        "Yeah!",
        "Hell yeah!",
        "Si",
        "Oui",
        "Hai",
        "Why yes",
        "Clearly",
    ];
    const NO: &[&str] = &[
        "No",
        "NO",
        "No way",
        "Hell no!",
        "Nay",
        "Absolutely not",
        "There's absolutely no way whatsoever",
        "No, no, and no",
        "Of course not",
        "No, but you already knew that",
        "Non",
        "iie",
        "Yeah, No",
        "It's a no from me",
    ];
    const MAYBE: &[&str] = &[
        "Maybe",
        "I'm busy now, try again later",
        "Huh, not sure",
        "Who knows",
        "Yes, but maybe not",
        "No, but maybe yes",
        "¯\\_(ツ)_/¯",
        "@deadshrugbot",
        "Are you kidding me?",
        "The answer lies within yourself",
    ];

    let mut rng = thread_rng();
    let roll = rng.gen_range(0..=9);

    match roll {
        0..=3 => choose_from(YES),
        4..=7 => choose_from(NO),
        8..=9 => choose_from(MAYBE),
        _ => "Only after fixing this bug",
    }
}

fn decide() -> &'static str {
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
        "No, but you already knew that",
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
        "I don't think",
        "Only if you win at /rps",
        "Only if you flip heads",
    ];

    let mut rng = thread_rng();
    let roll = rng.gen_range(0..=9);

    match roll {
        0..=3 => choose_from(YES),
        4..=7 => choose_from(NO),
        8..=9 => choose_from(MAYBE),
        _ => "Only after fixing this bug",
    }
}

fn dice(args: &str) -> String {
    let (dice_arg, format_arg) = args.split_once(' ').unwrap_or((args, "Rolled a {}"));
    let faces = dice_arg.trim().parse::<i64>().unwrap_or(6);
    if faces <= 0 {
        return "...".into();
    }
    let roll = thread_rng().gen_range(1..=faces);

    format_arg.replace("{}", faces.to_string());
}

fn rps() -> &'static str {
    let chosen = choose_from(&["Rock", "Paper", "Scissors"]);
    let dice = thread_rng().gen_range(0..10);
    match (chosen, dice) {
        ("Paper", 0) => "Super Paper",
        _ => chosen,
    }
}

fn rpsls() -> &'static str {
    choose_from(&["Rock", "Paper", "Scissors", "Lizard", "Spock"])
}

fn send_delete(chat_id: i64, message_id: i64) {
    let token = std::env::var("BOT_TOKEN").unwrap();
    let client = reqwest::blocking::Client::new();
    client
        .post(&format!(
            "https://api.telegram.org/bot{}/deleteMessage",
            token
        ))
        .header("Content-Type", "application/json")
        .body(
            json!({
                "chat_id": chat_id,
                "message_id": message_id,
            })
            .to_string(),
        )
        .send()
        .ok();
}

fn get_response(req: Request) -> Result<serde_json::Value, &'static str> {
    use serde_json::Value;
    let body: Value = match req.body() {
        now_lambda::Body::Binary(data) => {
            serde_json::from_slice(data).map_err(|_| "request body is not valid json")?
        }
        _ => return Err("Request body is not in binary format"),
    };

    let text = body["message"]["text"]
        .as_str()
        .or(body["message"]["caption"].as_str())
        .ok_or("neither body.message.text nor body.message.caption exist")?;

    let chat_id = body["message"]["chat"]["id"]
        .as_i64()
        .ok_or("body.message.chat.id does not exist")?;

    let message_id = body["message"]["message_id"]
        .as_i64()
        .ok_or("message.message_id not found");

    use BotResponse::*;
    match execute(&text) {
        Some(Message(text)) => Ok(json!({
            "method": "sendMessage",
            "chat_id": chat_id,
            "text": text,
        })),

        Some(LeaveChat) => Ok(json!({
            "method": "leaveChat",
            "chat_id": chat_id,
        })),

        Some(DeleteMessage) => Ok(json!({
            "method": "deleteMessage",
            "chat_id": chat_id,
            "message_id": message_id?,
        })),

        Some(DeleteAndSend(text)) => {
            send_delete(chat_id, message_id?);
            Ok(json!({
                "method": "sendMessage",
                "chat_id": chat_id,
                "text": text,
            }))
        }

        None => Err("command not found"),
    }
}

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    match get_response(req) {
        Ok(res) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(res.to_string())
            .expect("Something happened")),

        Err(e) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(e.into())
            .expect("Something happened")),
    }
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
