use rand::{prelude::*, random, seq::SliceRandom, thread_rng};
use serde_json::json;
use std::process::Command;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

fn choose_from<T: Copy>(a: &[T]) -> T {
    let i = thread_rng().gen_range(0..a.len());
    a[i]
}

fn join_with<'i, I, S>(mut it: I, sep: S) -> String
where
    I: Iterator<Item = &'i str>,
    S: Fn(usize) -> String,
{
    let mut res = it.next().map(|s| s.to_string()).unwrap_or_default();
    for (i, s) in it.enumerate() {
        res.push_str(&sep(i));
        res.push_str(s);
    }
    res
}

const BOTMENTION: &str = "@therngesusbot";

enum BotResponse {
    Message(String),
    Photo { url: String, caption: String },
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
        "/rpsg" => wrap!(rpsg(), Message),
        "/rpsls" => wrap!(rpsls(), Message),
        "/say" => wrap!(args.trim(), DeleteAndSend),
        "/anagram" => wrap!(anagram(args), Message),
        "/rick" => wrap!(rick(), DeleteAndSend),
        "/fortune" => wrap!(fortune(), Message),
        "/dart" => wrap!(dart(), Message),
        "/gato" | "/cat" => Some(cat()),

        "/test" => wrap!(
            format!("test {}", choose_from(&["failed", "succeeded"])),
            Message
        ),

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
        "Do {} yourself",
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
    template.replace("{}", chosen)
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
        "You have my blessing",
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
        "You have my blessing",
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

// /dice => dice 6
// /dice 10 => dice 10
// /dice 10,20 => dice 10
// /dice 10,20 Some {} Format {} => Some {dice 10} Format {dice 20}
// /dice 1,2 {} {} {} => {dice 1} {dice 2} {another dice 2}
fn dice(args: &str) -> String {
    let args = args.trim_start();
    let (dice_arg, format_arg) = args.split_once(' ').unwrap_or((args, "Rolled a {}"));
    let faces: Vec<_> = dice_arg
        .split(',')
        .map(|d| d.parse::<i64>().unwrap_or(6))
        .collect();
    if faces.iter().min().copied().unwrap_or(0) <= 0 {
        return "...".into();
    }

    let format_split = format_arg.split("{}");
    join_with(format_split, |i| {
        let f = faces.get(i).or(faces.last()).copied().unwrap_or(6);
        let roll = thread_rng().gen_range(1..=f);
        roll.to_string()
    })
}

fn rps() -> &'static str {
    let chosen = choose_from(&["Rock", "Paper", "Scissors"]);
    let super_paper = thread_rng().gen_ratio(2, 10);
    let rockscispaper = thread_rng().gen_ratio(5, 100);
    match (chosen, super_paper, rockscispaper) {
        (_, _, true) => "Rockscispaper",
        ("Paper", true, _) => "Super Paper",
        _ => chosen,
    }
}

fn rpsg() -> &'static str {
    choose_from(&["Rock", "Paper", "Scissors", "Gun"])
}

fn rpsls() -> &'static str {
    choose_from(&["Rock", "Paper", "Scissors", "Lizard", "Spock"])
}

fn anagram(arg: &str) -> String {
    let mut word: Vec<char> = arg.trim().chars().collect();
    if word.is_empty() {
        return "\u{AD} ".into(); // some invisible character
    }
    let slice = word.as_mut_slice();
    slice.shuffle(&mut thread_rng());
    slice.iter().copied().collect()
}

fn rick() -> String {
    let mut buf = [0u8; 11];

    let id = if thread_rng().gen_ratio(5, 10) {
        choose_from(&[
            "dQw4w9WgXcQ",
            "iik25wqIuFo",
            "uT6mKkkvjJY",
            "v7KafvXuqKE",
            "2xx_2XNxxfA",
        ])
    } else {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            abcdefghijklmnopqrstuvwxyz\
            0123456789\
            _-";

        for i in 0..11 {
            buf[i] = choose_from(ALPHABET);
        }
        while buf[10] == b'_' || buf[10] == b'-' {
            buf[10] = choose_from(ALPHABET);
        }

        std::str::from_utf8(&buf).unwrap()
    };

    format!("https://youtu.be/{}", id)
}

fn fortune() -> String {
    Command::new("fortune")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| String::from("Couldn't run `fortune` command sry ¯\\_(ツ)_/¯"))
}

fn dart() -> String {
    let mut rng = thread_rng();
    let lat: f32 = rng.gen_range(-90.0..=90.0);
    let lon: f32 = rng.gen_range(-180.0..=180.0);
    let lat_suf = if lat >= 0.0 { "N" } else { "S" };
    let lon_suf = if lon >= 0.0 { "E" } else { "W" };
    format!(
        "https://www.google.com/maps/place/{:.6}{}+{:.6}{}",
        lat.abs(),
        lat_suf,
        lon.abs(),
        lon_suf
    )
}

fn cat() -> BotResponse {
    BotResponse::Photo {
        url: "https://cataas.com/cat".into(),
        caption: "".into(),
    }
}

fn send_delete(chat_id: i64, message_id: i64) {
    let token = std::env::var("BOT_TOKEN").unwrap();
    let client = reqwest::blocking::Client::new();
    client
        .post(format!(
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
        Body::Binary(data) => {
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
    match execute(text) {
        Some(Message(text)) => Ok(json!({
            "method": "sendMessage",
            "chat_id": chat_id,
            "text": text,
        })),

        Some(Photo { url, caption }) => Ok(json!({
            "method": "sendPhoto",
            "chat_id": chat_id,
            "photo": url,
            "caption": caption,
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
                "disable_web_page_preview": true,
            }))
        }

        None => Err("command not found"),
    }
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    match get_response(req) {
        Ok(res) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(res.to_string().into())
            .expect("Couldn't create response")),

        Err(e) => Ok(Response::builder().status(StatusCode::OK).body(e.into())?),
    }
}

// Start the runtime with the handler
#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}
