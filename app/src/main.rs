#[macro_use] extern crate log;

mod commands;
mod database;
mod stream_notify;

use commands::{
    general::*,
    animals::{
        cat::*,
        dog::*
    },
    roll::*,
    user::{
        colour::*,
        title::*,
    }
};

use dotenv;
use serenity::{
    async_trait,
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
    client::validate_token,
    http::Http
};
use std::{
    collections::HashSet,
    env,
    error::Error
};
use serenity::client::ClientBuilder;
use std::sync::Arc;
use reqwest::redirect::Policy;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        debug!("Callback ready: {:?}", ready);
        let activity = Activity::playing("with your RNG tables");
        context.set_activity(activity);
        info!("{} is connected!", ready.user.name);
    }

    async fn presence_replace(&self,
                        context: Context,
                        new_vec: Vec<Presence>)
    {
        debug!("Callback presence_replace: {:?}", new_vec);
    }

    async fn presence_update(&self,
                       context: Context,
                       new: PresenceUpdateEvent)
    {
        debug!("Callback presence_update: {:?}", new.presence);
        // stream_notify::handler(context, new);
    }
}

#[group("General")]
#[description = "General commands"]
#[commands(roll20, roll)]
struct General;

#[group("Animals")]
#[description = "Commands which are for getting pics of animals"]
#[commands(roll20, roll)]
struct Animals;

#[group("User")]
#[description = "Commands which are for the user"]
#[commands(roll20, roll)]
struct User;

// group!({
//     name: "general",
//     options: {},
//     commands: [
//         roll20,
//         roll,
//     ],
// });

// group!({
//     name: "animals",
//     options: {},
//     commands: [
//     cat,
//     dog,
//     ],
// });
//
// group!({
//     name: "user",
//     options: {},
//     commands: [
//         title,
//         colour,
//     ],
// });

// #[help]
// #[individual_command_tip =
// "Hello! こんにちは！Hola! Bonjour! 您好!\n\
// If you want more information about a specific command, just pass the command as argument."]
// #[command_not_found_text = "Could not find: `{}`."]
// #[max_levenshtein_distance(3)]
// #[indention_prefix = "+"]
// #[lacking_permissions = "Hide"]
// #[lacking_role = "Nothing"]
// #[wrong_channel = "Strike"]
// async fn my_help(
//     context: &mut Context,
//     msg: &Message,
//     args: Args,
//     help_options: &'static HelpOptions,
//     groups: &[&'static CommandGroup],
//     owners: HashSet<UserId>
// ) -> CommandResult {
//     help_commands::with_embeds(context, msg, args, help_options, groups, owners)
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv::from_filename("mount/env")
        .expect("Failed to load mount/env file");

    env_logger::init();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Do any database schema migration work before starting the Discord client
    let database = database::Handle::new();
    database.update_schema()
        .expect("Couldn't update database schema, giving up");

    // let mut client = Client::new(
    //     &token,
    //     Handler
    // )
    // .expect("Error creating client");

    // let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
    //     Ok(info) => {
    //         let mut owners = HashSet::new();
    //         owners.insert(info.owner.id);
    //
    //         (owners, info.id)
    //     },
    //     Err(why) => panic!("Could not access application info: {:?}", why),
    // };

    match validate_token(&token) {
        Ok(_) => info!("Token successfully validated. Continuing."),
        Err(_) => {
            error!("Token was not successfully validated. Cannot continue.");
            return Ok(());
        }
    }

    let http = Http::new_with_token(&token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => {
            error!("Unable to retrieve application info: {:?}", why);
            return Ok(());
        }
    };

    let framework = StandardFramework::new()
        .configure(|configuration| {
            configuration
                .with_whitespace(false)
                .on_mention(Some(bot_id))
                .prefix("!")
                .delimiters(vec![", ", ","])
                .owners(owners)
                .ignore_webhooks(false)
                .ignore_bots(true)
                .case_insensitivity(true)
        })
        .before(|_context, msg, command_name| {
            debug!("Got command '{}' by user '{}'",
                   command_name,
                   msg.author.name);
            true
        })
        .after(|_context, _msg, command_name, error| {
            match error {
                Ok(()) => debug!("Processed command '{}'", command_name),
                Err(why) => error!("Command '{}' returned error {:?}", command_name, why),
            }
        })
        //.prefix_only(prefix_only) // Maybe Use
        .unrecognised_command(|_, _, unknown_command_name| {
            debug!("Could not find command named '{}'", unknown_command_name);
        })
        // Code to execute when commands fail to dispatch
        .on_dispatch_error(|_context, msg, error| {
            debug!("Failed to dispatch `{}`: {:?}", msg.content, error);
        })
        .group(&GENERAL_GROUP)
        .group(&ANIMALS_GROUP)
        .group(&USER_GROUP);
        //.help(&HELP);

    let mut client = ClientBuilder::new(&token)
        .event_handler(Handler)
        //.intents(GatewayIntents::all())
        .framework(framework)
        //.register_songbird()
        .await?;

    {
        let mut data = client.data.write().await;
        //
        // let url = configuration.bot.database.url;
        // let pool = PgPoolOptions::new().max_connections(20).connect(&url).await?;
        // let http_client = Client::builder().user_agent(REQWEST_USER_AGENT).redirect(Policy::none()).build()?;
        //
        // data.insert::<DatabasePool>(pool);
        // data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        // data.insert::<ReqwestContainer>(http_client);
    }

    if let Err(why) = client.start_autosharded().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }

    Ok(())

    // client.with_framework(
    //     StandardFramework::new()
    //     .configure(|c| c
    //         .with_whitespace(false)
    //         .on_mention(Some(bot_id))
    //         .prefix("!")
    //         .delimiters(vec![", ", ","])
    //         .owners(owners)
    //     )
    //     // Code to execute before a command execution
    //     .before(|_context, msg, command_name| {
    //         debug!("Got command '{}' by user '{}'",
    //                  command_name,
    //                  msg.author.name);
    //         true
    //     })
    //     // Code to execute after a command execution
    //     .after(|_context, _msg, command_name, error| {
    //         match error {
    //             Ok(()) => debug!("Processed command '{}'", command_name),
    //             Err(why) => error!("Command '{}' returned error {:?}", command_name, why),
    //         }
    //     })
    //     // Code to execute whenever an attempted command-call's
    //     // command could not be found
    //     .unrecognised_command(|_, _, unknown_command_name| {
    //         debug!("Could not find command named '{}'", unknown_command_name);
    //     })
    //     // Code to execute when commands fail to dispatch
    //     .on_dispatch_error(|_context, msg, error| {
    //         debug!("Failed to dispatch `{}`: {:?}", msg.content, error);
    //     })
    //     .help(&MY_HELP)
    //     .group(&GENERAL_GROUP)
    //     .group(&ANIMALS_GROUP)
    //     .group(&USER_GROUP)
    // );
}
