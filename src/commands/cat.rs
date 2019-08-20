extern crate reqwest;

use serde::{Deserialize};

use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
    utils::MessageBuilder,
};

#[derive(Debug, Deserialize)]
struct RootCat {
    cat: Cat
}

#[derive(Debug, Deserialize)]
struct Cat {
    #[serde(default)]
    breeds: Vec<::serde_json::Value>,
    #[serde(default)]
    categories: Vec<::serde_json::Value>,
    height: i64,
    id: String,
    url: String,
    width: i64,
}

#[command]
// Cat Pic
fn cat(context: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    debug!("specificcat command handler called.");
    debug!("Message: {:?}", msg);
    debug!("Args: {:?}", args);

    let baseurl = "https://api.thecatapi.com/v1/images/";
    let mut path;

    if !args.is_empty() {
        path = args.single::<String>().unwrap();
    } else {
        path = String::from("search");
    }
    debug!("{}", path);

    let callurl = format!("{}{}", baseurl, path);

    let resp : RootCat = reqwest::get(callurl.as_str())?.json()?;
    debug!("{:#?}", resp);

    let response = MessageBuilder::new()
        .push_bold_safe(&msg.author)
        .push(" Meow: ")
        .push(resp.cat.url)
        .build();

    if let Err(why) = msg.channel_id.say(&context.http, &response) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}