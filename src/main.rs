#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate reqwest;
extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;

mod errors;

use rocket_contrib::Json;

use errors::*;


static TELEGRAM_API: &'static str = "https://api.telegram.org/bot";
static ERROR_MSG: &'static str = "……";

#[derive(Serialize, Deserialize)]
struct Chat {
    id: u32,
    #[serde(rename = "type")]
    chat_type: String
}


#[derive(Serialize, Deserialize)]
struct Message {
    message_id: u32,
    chat: Chat,
    text: String
}


#[derive(Serialize, Deserialize)]
struct Update {
    update_id: u32,
    message: Message
}

#[derive(Serialize, Deserialize)]
struct Answer {
    answer: String,
    #[serde(rename = "question:")]
    question: String
}


fn send_message(text: &str, chat_id: u32) -> Result<()> {
    let tg_bot_token: &'static str = env!("TG_BOT_TOKEN");
    let url = TELEGRAM_API.to_owned() + tg_bot_token + "/sendMessage";
    let _ = reqwest::Client::new()?
    .post(&url)?
    .json(&json!({"text": text, "chat_id": chat_id}))?
    .send()?;
    Ok(())
}

fn get_answer(question: &str) -> String {
    let bosonask_api: &'static str = env!("BOSONASK_API");
    reqwest::Client::new()
        .expect(&ERROR_MSG)
        .post(bosonask_api)
        .expect(&ERROR_MSG)
        .json(&json!({"q": question}))
        .expect(&ERROR_MSG)
        .send()
        .expect(&ERROR_MSG)
        .json::<Answer>()
        .expect(&ERROR_MSG)
        .answer
}

#[post("/", format = "application/json", data = "<update>")]
fn handle_update(update: Json<Update>) -> Result<()> {
    let answer = get_answer(&update.0.message.text);
    let _ = send_message(&answer, update.0.message.chat.id);
    Ok(())
}

fn main() {
    rocket::ignite().mount("/", routes![handle_update]).launch();
}
