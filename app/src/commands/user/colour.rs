use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandError,
        CommandResult,
        macros::command,
    },
    utils::MessageBuilder,
};

// Command to change a users colour in the guild using currently available
// roles, prefixed with `colour-`. 
#[command]
#[description = "Sets your colour using designated roles."]
#[usage = "`!colour <name of colour>`"]
#[only_in(guilds)]
async fn colour(ctx: &Context, msg: &Message) -> CommandResult {
    debug!("colour command handler called");
    let args: Vec<&str> = msg.content.split(' ').skip(1).collect();

    let colour =  match args.get(0) {
        None => {
            // This is a usage error, not a bot failure
            respond(ctx, msg, "you forgot to pick a colour; e.g. `!colour blue`");
            return Ok(())
        },
        _ => "colour-".to_owned() + &args.join("").to_lowercase()
    };

    let p_guild = match msg.guild_id {
        None => {
            let txt = "we couldn't find which guild this came from. Sorry!";
            respond(ctx, msg, txt);
            return Err(CommandError(String::from(txt)))
        },
        Some(guild) => {
            let p_guild = match guild.to_partial_guild(&ctx.http) {
                Ok(p_guild) => p_guild,
                Err(_) => {
                    let txt = "something went wrong when fetching guild info. Sorry!";
                    respond(ctx, msg, txt);
                    return Err(CommandError(String::from(txt)))
                }
            };
            p_guild
        }
    };

    let mut member = match p_guild.member(&ctx, &msg.author) {
        Ok(member) => member,
        Err(_) => {
            let txt = "I can't find you in the guild. Sorry!";
            respond(ctx, msg, txt);
            return Err(CommandError(String::from(txt)))
        }
    };

    let role_id = match p_guild.role_by_name(&colour) {
        Some(role_id) => role_id,
        None => {
            let txt = colour + " isn't available. Sorry!";
            respond(ctx, msg, &txt);
            return Err(CommandError(String::from(txt)))
        }
    };

    let removals: Vec<RoleId> = match member.roles(&ctx.cache) {
        None => Vec::new(),
        Some(roles) => {
            roles.iter()
                .filter(|role| role.name.starts_with("colour-"))
                .map(|role| role.id)
                .collect()
        }
    };

    if removals.len() > 0 {
        if let Err(why) = member.remove_roles(&ctx.http, &removals) {
            error!("Error removing roles: {:?}", why);
            let txt = "we couldn't remove your old colours. Sorry!";
            respond(ctx, msg, txt);
            return Err(CommandError(String::from(txt)))
        };
    }

    match member.add_role(&ctx.http, role_id) {
        Ok(_) => {
            let txt = "your colour has been updated!";
            respond(ctx, msg, txt);

            Ok(())
        },
        Err(_) => {
            let txt = "we couldn't give you this colour. Sorry!";
            respond(ctx, msg, txt);
            return Err(CommandError(String::from(txt)))
        }
    }
}

// Sends a response to a user's message
fn respond(ctx: &Context, msg: &Message, txt: &str) {
    let response = MessageBuilder::new()
        .push_bold_safe(&msg.author)
        .push(", ")
        .push(txt)
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
        error!("Error sending message: {:?}", why);
    }
}