use std::env;

use regex::Regex;

use reqwest;

use futures::StreamExt;
use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, Message, MessageKind, UpdateKind};

const URL: &str = "https://cb24154236a0.ngrok.io";

async fn start_message(api: Api, message: Message) -> Result<(), Box<dyn std::error::Error>> {
    api.send(message.text_reply(format!("Suuuuppp"))).await?;
    Ok(())
}

async fn ping_message(api: Api, message: Message) -> Result<(), Box<dyn std::error::Error>> {
    let res = ping().await?;
    let status_code = res.status();
    api.send(message.text_reply(format!("Status server: {}", status_code))).await?;
    Ok(())
}

async fn commands(api: Api, message: Message) -> Result<(), Box<dyn std::error::Error>> {
    let (_command, _data): (String, String) = match parse_command(&message).await? {
        Some(t) => {
            let command = &t.0;

            match command.as_str() {
                "/start" => start_message(api, message).await?,
                "/start@jureba_bot" => start_message(api, message).await?,
                "/ping" => ping_message(api, message).await?,
                "/ping@jureba_bot" => ping_message(api, message).await?,
                _ => ()
            };
            t
        },
        None => (String::from("Opa"), String::from("Agora"))
    };
    
    Ok(())
}

async fn ping() -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let status = reqwest::get(&*URL).await?;
    Ok(status)
}

async fn parse_command(message: &Message) -> Result<Option<(String, String)>, Error> {

    match message.chat {
        telegram_bot::MessageChat::Private(_) => {
            match message.kind {
                MessageKind::Text { ref data, ..} => {
                    let _re = match Regex::new(r"/([a-z]\w+) {0}") {
                        Ok(re) => {
                            let _t = match re.find(data) {
                                Some(t) => {
                                    let command = data[..t.end()].to_string();
                                    let message = data[t.end()..].to_string();
                                    return Ok(Some((String::from(command), String::from(message))));
                                }
                                None => {}
                            };
                            return Ok(None);
                        },
                        Err(_) => {
                            return Ok(None);
                        }
                    };
                },
                _ => Ok(None)
            }
        },
        telegram_bot::MessageChat::Group(_) => {
            
            match message.kind {
                MessageKind::Text { ref data, ..} => {
                    let _re = match Regex::new(r"/([a-z]\w+@[a-z]\w+) {0}") {
                        Ok(re) => {
                            let _t = match re.find(data) {
                                Some(t) => {
                                    let command = data[..t.end()].to_string();
                                    let message = data[t.end()..].to_string();
                                    return Ok(Some((String::from(command), String::from(message))));
                                }
                                None => {}
                            };
                            return Ok(None);
                        },
                        Err(_) => {
                            return Ok(None);
                        }
                    };
                },
                _ => Ok(None)
            }
        },
        _ => Ok(None)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let api = Api::new(token);
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            commands(api.clone(), message).await?;
        }
    }

    Ok(())
}