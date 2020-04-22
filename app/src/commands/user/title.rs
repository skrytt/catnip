use crate::database;
use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandError,
        CommandResult,
        macros::command,
    },
    utils::MessageBuilder,
    utils::parse_emoji,
};

const MAX_TITLE_LENGTH: usize = 128;

#[command]
#[description = "Gets, sets or clears your title."]
#[usage = "`!title`, `!title set ...` or `!title clear`."]
fn title(ctx: &mut Context, msg: &Message) -> CommandResult {
    debug!("title command handler called");

    let args: Vec<&str> = msg.content.split(' ').collect();

    match args.get(1) {
        None => handle_get_title(ctx, msg),
        Some(&"set") => {
            match args.get(2..args.len()) {
                None => {
                    let response = MessageBuilder::new()
                        .push_bold_safe(&msg.author)
                        .push(", Usage: `!title set The Fabulous`")
                        .build();

                    if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
                        error!("Error sending message: {:?}", why);
                    }

                    // This is a usage error, not a bot failure
                    Ok(())
                },
                Some(title_args) => {
                    // Check for custom emojis
                    if has_custom_emoji(Some(title_args)) {
                        let response = MessageBuilder::new()
                            .push_bold_safe(&msg.author)
                            .push(", custom Emojis are not allowed in titles")
                            .build();

                        if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
                            error!("Error sending message: {:?}", why);
                        }

                        // This is a usage error, not a bot failure
                        return Ok(())
                    }

                    let new_title = title_args.join(" ");
                    handle_set_title(ctx, msg, Some(new_title))
                },
            }
        },
        Some(&"clear") => handle_set_title(ctx, msg, None),
        Some(_) => {
            // Unrecognised argument
            let response = MessageBuilder::new()
                .push_bold_safe(&msg.author)
                .push(", unrecognised !title subcommand. ")
                .push("use `!title`, `!title set ...` or `!title clear`.")
                .build();

            if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
                error!("Error sending message: {:?}", why);
            }
            // User error
            return Ok(())
        }
    }
}

fn handle_get_title(
    ctx: &mut Context,
    msg: &Message,
) -> CommandResult
{
    // User ID of the user executing the command
    let user_id = msg.author.id;

    debug!("User DB data retrieval...");
    let database = database::Handle::new();
    let user: database::User = match database.user(user_id.0)
        {
            Err(_) => {
                let reason = String::from("Could not retrieve user data from database");
                error!("{}", reason);
                return Err(CommandError(reason))
            },
            Ok(data) => data,
        };

    let response = match user.title {
        None => MessageBuilder::new()
            .push_bold_safe(&msg.author)
            .push(", you don't have a title! Use `!title set ...` to set one.")
            .build(),
        Some(title) => MessageBuilder::new()
            .push_bold_safe(&msg.author)
            .push(", your title is ")
            .push_bold(title)
            .push(".")
            .build(),
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}

fn handle_set_title(
    ctx: &mut Context,
    msg: &Message,
    title: Option<String>
) -> CommandResult
{
    // Nothing to validate if title is None
    let title: Option<String> = match title {
        None => None,
        Some(title) => {
            debug!("Formatting title {:?}...", title);
            let title = String::from(title.trim_matches(' '));
            if title.is_empty() {
                // Empty, or had emojis or other characters removed by to_snake_case
                let response = MessageBuilder::new()
                    .push_bold_safe(&msg.author)
                    .push(", sorry, that didn't work. Try a different title!")
                    .build();
                if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
                    error!("Error sending message: {:?}", why);
                }
                // User error
                return Ok(())
            }
            if title.len() > MAX_TITLE_LENGTH {
                // Title is too long
                let response = MessageBuilder::new()
                    .push_bold_safe(&msg.author)
                    .push(", please choose a shorter title!")
                    .build();
                if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
                    error!("Error sending message: {:?}", why);
                }
                // User error
                return Ok(())
            }
            debug!("Resulting title: {:?}", title);
            Some(title)
        }
    };

    // User ID of the user executing the command
    let user_id = msg.author.id;

    debug!("User DB data retrieval...");
    let database = database::Handle::new();
    let mut user: database::User = match database.user(user_id.0)
        {
            Err(_) => {
                let reason = String::from("Could not retrieve user data from database");
                error!("{}", reason);
                return Err(CommandError(reason))
            },
            Ok(data) => data,
        };

    user.title = title.clone();

    debug!("Updating user DB entry...");
    if let Err(_) = database.user_update(
        user_id.0,
        &user
    ) {
        let reason = String::from("Could update user data in database");
        error!("{}", reason);
        return Err(CommandError(reason))
    }
    debug!("Updated user title in database to '{:?}'", user.title);

    let response = match title {
        Some(title) => MessageBuilder::new()
            .push_bold_safe(&msg.author)
            .push(", set your title to ")
            .push_bold(title)
            .push("!")
            .build(),
        None => MessageBuilder::new()
            .push_bold_safe(&msg.author)
            .push(", cleared your title!")
            .build(),
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}

fn has_custom_emoji(
    title: Option<&[&str]>
) -> bool
{

    for (i, item) in title.iter().enumerate() {
        for(ie, title_part) in item.iter().enumerate() {
            debug!("The {}th item is {:?}", i+1, title_part);

            // if it can be parsed, it is an custom emoji
            if let Some(emoji) = parse_emoji(title_part) {
                debug!("Emoji Info, id:{:?}, name:{:?}", emoji.id, emoji.name);
                return true;
            }
        }
        debug!("The {}th item is {:?}", i+1, item);
    }

    return false;
}
