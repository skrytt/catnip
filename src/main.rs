#[macro_use] extern crate log;

mod commands;
mod database;
mod stream_notify;

use commands::{
    general::*,
    cat::cat::*,
};
use dotenv::dotenv;
use serenity::{
    prelude::*,
    framework::standard::{
        Args, CommandResult, CommandGroup,
        HelpOptions, help_commands, StandardFramework,
        macros::{group, help},
    },
    model::{
        channel::Message,
        event::PresenceUpdateEvent,
        gateway::{Activity, Presence, Ready},
        id::UserId,
    },
};
use std::{
    collections::HashSet,
    env,
};

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, context: Context, ready: Ready) {
        debug!("Callback ready: {:?}", ready);
        let activity = Activity::playing("with your RNG tables");
        context.set_activity(activity);
        info!("{} is connected!", ready.user.name);
    }

    fn presence_replace(&self,
                        _context: Context,
                        new_vec: Vec<Presence>)
    {
        debug!("Callback presence_replace: {:?}", new_vec);
    }

    fn presence_update(&self,
                       context: Context,
                       new: PresenceUpdateEvent)
    {
        debug!("Callback presence_update: {:?}", new.presence);
        stream_notify::handler(context, new);
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
