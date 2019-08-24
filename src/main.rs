#[macro_use] extern crate log;

mod commands;
mod database;

use serenity::{
    framework::standard::{
        Args, CommandResult, CommandGroup,
        HelpOptions, help_commands, StandardFramework,
        macros::{group, help},
    },
    model::{
        channel::Message,
        event::PresenceUpdateEvent,
        gateway::{Activity, ActivityType, Ready},
        id::UserId,
    },
};

use std::{
    collections::HashSet,
    env,
};
use serenity::prelude::*;
use commands::{
    general::*,
    cat::cat::*,
};
use dotenv::dotenv;
use time;

const STREAM_ADVERTISE_COOLDOWN: i64 = 21600; // 6 hours

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, context: Context, ready: Ready) {
        let activity = Activity::playing("with your RNG tables");
        context.set_activity(activity);

        info!("{} is connected!", ready.user.name);
    }

    fn presence_update(&self,
                       context: Context,
                       new: PresenceUpdateEvent)
    {
        debug!("Presence update, new.presence: {:?}", new.presence);

        // Stream start detection
        let streaming_activity = match new.presence.activity {
            None => return,
            //Some(activity) => match activity.kind {
            //    ActivityType::Streaming => activity,
            //    _ => return,
            //},

            // For testing only
            Some(activity) => activity
        };

        debug!("DB path check...");
        let db_path = match env::var("DATABASE_PATH") {
            Err(_) => {
                error!("DATABASE_PATH is not set in the environment");
                return
            }
            Ok(path) => path,
        };
        let database = database::Handle::new(&db_path);

        debug!("User retrieval...");
        let discord_user_id = new.presence.user_id;

        match context.cache.read().user(discord_user_id) {
            None => {
                error!("Failed to get Discord user object from Serenity cache");
                return
            },
            Some(user) => {
                let user = user.read();
                debug!("Member {} ({}#{}) presence update with activity name '{:?}'",
                       user.id,
                       user.name,
                       user.discriminator,
                       streaming_activity.name,
                );
            },
        };
        match database.user(discord_user_id.0) {
            Err(_) => {
                error!("Could not retrieve user data from database");
                return
            },
            Ok(Some(_)) => (),
            Ok(None) => {
                if let Err(_) = database.user_update(
                    discord_user_id.0, &Default::default()
                ) {
                    error!("Could not add user data to database");
                    return
                }
            },
        };

        debug!("Guild retrieval...");
        let discord_guild_id = match new.guild_id {
            None => {
                debug!("Got presence update with no discord guild ID");
                return
            },
            Some(guild_id) => guild_id,
        };
        match database.guild(discord_guild_id.0) {
            Err(_) => {
                error!("Could not retrieve guild data from database");
                return
            },
            Ok(Some(_)) => (),
            Ok(None) => {
                if let Err(_) = database.guild_update(
                    discord_guild_id.0, &Default::default()
                ) {
                    error!("Could not add guild data to database");
                    return
                }
            }
        };

        debug!("Member retrieval...");
        let mut member: database::Member = match database.member(
            discord_guild_id.0, discord_user_id.0)
        {
            Err(_) => {
                error!("Could not retrieve member data from database");
                return
            },
            Ok(Some(data)) => data,
            Ok(None) => Default::default(),
        };

        debug!("member data: {:?}", member);
        let this_timestamp = time::get_time().sec;
        if this_timestamp - member.last_stream_notify_timestamp > STREAM_ADVERTISE_COOLDOWN {
            member.last_stream_notify_timestamp = this_timestamp;
            if let Err(_) = database.member_update(
                discord_guild_id.0,
                discord_user_id.0,
                &member
            ) {
                error!("Could not add member data to database");
                return
            }
            debug!("Updated member timestamp and would shout out stream!");
        } else {
            debug!("Last stream too recent; would not shout out stream");
        }
    }
}

group!({
    name: "general",
    options: {},
    commands: [roll20],
});

group!({ 
    name: "cat",
    options: {},
    commands: [cat],
});

#[help]
#[individual_command_tip =
"Hello! こんにちは！Hola! Bonjour! 您好!\n\
If you want more information about a specific command, just pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

fn main() {
    dotenv()
        .expect("Failed to load .env file");

    env_logger::init();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(
        &token,
        Handler
    )
    .expect("Error creating client");

    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
        .configure(|c| c
            .with_whitespace(false)
            .on_mention(Some(bot_id))
            .prefix("!")
            .delimiters(vec![", ", ","])
            .owners(owners)
        )
        // Code to execute before a command execution
        .before(|_context, msg, command_name| {
            debug!("Got command '{}' by user '{}'",
                     command_name,
                     msg.author.name);
            true
        })
        // Code to execute after a command execution
        .after(|_context, _msg, command_name, error| {
            match error {
                Ok(()) => debug!("Processed command '{}'", command_name),
                Err(why) => error!("Command '{}' returned error {:?}", command_name, why),
            }
        })
        // Code to execute whenever an attempted command-call's
        // command could not be found
        .unrecognised_command(|_, _, unknown_command_name| {
            debug!("Could not find command named '{}'", unknown_command_name);
        })
        // Code to execute when commands fail to dispatch
        .on_dispatch_error(|_context, msg, error| {
            debug!("Failed to dispatch `{}`: {:?}", msg.content, error);
        })
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&CAT_GROUP)
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
