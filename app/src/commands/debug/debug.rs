use crate::database;

use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command, CommandError
    },
    utils::{MessageBuilder, Colour},
};
use std::borrow::Borrow;
use std::time::SystemTime;

// Debug Twitch Shoutout Message
#[command]
#[owners_only]
fn debug(context: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    debug!("twitchdebug command handler called.");
    // debug!("Message: {:?}", msg);
    // debug!("Args: {:?}", args);

    let channel = msg.channel_id;

    // face gets pfp url or just discords default URL for pfps
    let authorURL = msg.author.face();

    let guild_id = match msg.guild_id {
        None => {
            let txt = "Call not made in a Guild Channel";
            return Err(CommandError(String::from(txt)))}
        Some(guild_id) => guild_id
    };

    debug!("{:?}" ,authorURL);

    let cached_member = match context.cache.read().member(guild_id, msg.author.id) {
        None => {
            let txt = "Member couldn't be found within cache";
            return Err(CommandError(String::from(txt)))},
        Some(cached_member) => cached_member,
    };

    let member_colour = match cached_member.colour(context.cache.borrow()) {
        // If no colour use the default colour (no clue when this would be the case)
        None => {Colour::default()},
        Some(member_colour) => member_colour,
    };

    let response = MessageBuilder::new()
        .push_bold_safe("Exceeds")
        .push(" is streaming ")
        .push_bold_safe("The Outer Worlds ")
        .push("https://www.twitch/scrubceeds")
        .build();

    //TODO: Look at error handling
    if let Err(why) = channel.send_message(&context, |m| {
        m.content(response);
        m.embed(|e|
            e.title("Stream Title here") // Stream Title
                .colour(member_colour)
                .url("https://www.twitch/scrubceeds") // Stream URL
                // .description("self.summary.to_string()") // Not needed?
                //.thumbnail("https://i.imgur.com/oSJQ4Sc.jpg")
                 .author(|a| {
                     a.name(&msg.author.name)
                        .icon_url(authorURL)
                 })
                .field("Playing", "The Outer Worlds", true) // Game being Played
                //.field("PlaceHolder", "PlaceHolder", true) // Maybe use?
                //.image("https://i.imgur.com/oSJQ4Sc.jpg")
                .footer(|f| f.text(format!("Stream started at {:?}", SystemTime::now()))) // Point out stream starting time
                //.fields(self.create_fields(false))
        )
    }) {error!("Error sending message: {:?},", why)};

    Ok(())
}

#[command]
#[owners_only]
fn debugdbupdate(context: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let database = database::Handle::new();

    let guild_id = match msg.guild_id {
        None => {
            let txt = "Call not made in a Guild Channel";
            return Err(CommandError(String::from(txt)))}
        Some(guild_id) => guild_id
    };


    debug!("Member DB data retrieval...");
    let member: database::Member = match database.member(
        guild_id.0, msg.author.id.0)
        {
            Err(_) => {
                let txt= "Could not retrieve member data from database";
                return Err(CommandError(String::from(txt)));
            },
            Ok(data) => data,
        };

    debug!("User DB data retrieval...");
    let user: database::User = match database.user(msg.author.id.0)
        {
            Err(_) => {
                let txt=  "Could not retrieve user data from database";
                return Err(CommandError(String::from(txt)));
            },
            Ok(data) => data,
        };

    // Update the timestamp of the last shout-out in the database
    let mut member = member.clone();
    member.last_stream_notify_timestamp = time::get_time().sec;

    debug!("guild_id: {:?}, user_id: {:?}, member: {:?}", guild_id.0, msg.author.id.0, member.last_stream_notify_timestamp);

//    if let Err(_) = database.member_update(
//        guild_id.0,
//        user_id.0,
//        &member
//    ) {
//        let txt = "Couldn't update member data in database");
//        return Err(CommandError(String::from(txt)));
//    }
    let update = match database.member_update(
        guild_id.0,
        msg.author.id.0,
        &member,
    ) {
        Ok(update) => update,
        Err(_) => {
            let txt=  "Couldn't update member data in database";
            return Err(CommandError(String::from(txt)));
        },
    };

    Ok(())
}