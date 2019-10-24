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
#[usage = "`!colour name of colour`"]
fn colour(ctx: &mut Context, msg: &Message) -> CommandResult {
    debug!("colour command handler called");
    let args: Vec<&str> = msg.content.split(' ').collect();

    match args.get(1) {
        None => {
            let response = MessageBuilder::new()
                .push_bold_safe(&msg.author)
                .push(", Usage: `!colour blue`")
                .build();

            if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
                error!("Error sending message: {:?}", why);
            }

            // This is a usage error, not a bot failure
            Ok(())
        },
        _ => {
            let colour: &str = &("colour-".to_owned() + &args.join("").to_lowercase());
            match msg.guild_id {
                None => {
                    let response = MessageBuilder::new()
                        .push_bold_safe(&msg.author)
                        .push(", we couldn't find which guild this came from. Sorry!")
                        .build();

                    if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
                        error!("Error sending message: {:?}", why);
                    };

                    // This is not a bot failure
                    Ok(())
                },
                Some(guild) => {
                    let p_guild = match guild.to_partial_guild(&ctx.http) {
                        Ok(guild) => guild,
                        Err(_) => return Err(CommandError(msg.author.name.to_owned() + ", something went wrong when fetching guild info. Sorry!"))
                    };

                    let mut member = match guild.member(&ctx, &msg.author) {
                        Ok(member) => member,
                        Err(_) => return Err(CommandError(msg.author.name.to_owned() + ", I can't find you in the guild. Sorry!"))
                    };

                    let role_id = match p_guild.role_by_name(colour) {
                        Some(role_id) => role_id,
                        None => return Err(CommandError(msg.author.name.to_owned() + ", the colour " + colour + " isn't available. Sorry!"))
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
                            return Err(CommandError(msg.author.name.to_owned() + ", we couldn't remove your old colours. Sorry!"));
                        };
                    }

                    match member.add_role(&ctx.http, role_id) {
                        Ok(_) => {
                            let response = MessageBuilder::new()
                                .push_bold_safe(&msg.author.name)
                                .push(", your colour has been updated!")
                                .build();

                            if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
                                error!("Error sending message: {:?}", why);
                            }   

                            Ok(())
                        },
                        Err(_) => Err(CommandError(msg.author.name.to_owned() + ", we couldn't give you this colour. Sorry!"))
                    }
                }
            }
        }
    }
}
