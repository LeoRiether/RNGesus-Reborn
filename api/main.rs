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
        "/rps" => Some(rps().into()),
        "/rpsls" => Some(rpsls().into()),
        "/say" => Some(args.trim().into()),

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

    match roll {
        0..=3 => choose_from(YES),
        4..=7 => choose_from(NO),
        8..=9 => choose_from(MAYBE),
        _ => "Only after fixing this bug",
    }
}

fn dice(args: &str) -> String {
    let first_arg = args.split_ascii_whitespace().next().unwrap_or_default();
    let faces = first_arg.trim().parse::<i64>().unwrap_or(6);
    if faces <= 0 {
        return "...".into();
    }
    let roll = thread_rng().gen_range(1..=faces);
    format!("Rolled a {}", roll)
}

fn rps() -> &'static str {
    let chosen = choose_from(&["Rock", "Paper", "Scissors"]);
    let dice = thread_rng().gen_range(0..33);
    match (chosen, dice) {
        ("Paper", 0) => "Super Paper",
        _ => chosen,
    }
}

fn rpsls() -> &'static str {
    choose_from(&["Rock", "Paper", "Scissors", "Lizard", "Spock"])
}

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    use serde_json::Value::{self, Number, String};

    macro_rules! err {
        ($reason:expr) => {
            return Ok(Response::builder()
                .status(StatusCode::OK)
                .body($reason.to_string())
                .expect("Something went wrong"))
            // return Err(NowError::new($reason))
        };
    }

    let body: Value = match req.body() {
        now_lambda::Body::Binary(data) => match serde_json::from_slice(data) {
            Ok(body) => body,
            Err(_) => err!("couldn't parse json"),
        },
        _ => err!("Request body is not in binary format"),
    };

    let text = match (&body["message"]["text"], &body["message"]["caption"]) {
        (String(x), _) | (_, String(x)) => x,
        _ => err!("either body.message.text or body.message.caption do not exist"),
    };

    let chat_id = match &body["message"]["chat"]["id"] {
        Number(x) => x.as_i64().unwrap(),
        _ => err!("body.message.chat.id does not exist"),
    };

    let response_text = match execute(&text) {
        Some(res) => res,
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
