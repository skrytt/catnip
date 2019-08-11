use rand::{Rng, thread_rng};

use serenity::{
    framework::standard::{
        Args, CommandResult, CommandGroup,
        HelpOptions, help_commands, StandardFramework,
        macros::{command, group, help},
    },
    model::{channel::Message, gateway::Ready, id::UserId},
    utils::MessageBuilder,
};

use std::{
    collections::HashSet,
    env,
};
use serenity::prelude::*;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

group!({
    name: "general",
    options: {},
    commands: [roll20],
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
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Error creating client");

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
            .owners(owners))

        // Code to execute before a command execution
        .before(|_context, msg, command_name| {
            println!("Got command '{}' by user '{}'",
                     command_name,
                     msg.author.name);
            true
        })

        // Code to execute after a command execution
        .after(|_context, _msg, command_name, error| {
            match error {
                Ok(()) => println!("Processed command '{}'", command_name),
                Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
            }
        })
        // Code to execute whenever an attempted command-call's
        // command could not be found
        .unrecognised_command(|_, _, unknown_command_name| {
            println!("Could not find command named '{}'", unknown_command_name);
        })
        // Code to execute when commands fail to dispatch
        .on_dispatch_error(|_context, _msg, error| {
            println!("{:?}", error);
        })
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

#[command]
/// Roll 1d20 and send a response with the result.
fn roll20(context: &mut Context, msg: &Message) -> CommandResult {

    // The RNG function produces values in the range 0 to 20-1, so
    // add 1 to bring the range into the expected range.
    let rolled_value: i32 = thread_rng().gen_range(0, 20) + 1;

    let response = MessageBuilder::new()
        .push_bold_safe(&msg.author)
        .push(" rolls 1d20 with the result: ")
        .push_bold(rolled_value)
        .build();

    if let Err(why) = msg.channel_id.say(&context.http, &response) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
