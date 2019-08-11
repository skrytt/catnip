use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // This is called when a shard is booted, and a READY payload is sent
    // by Discord. This payload contains data like the current user's guild Ids,
    // current user data, private channels, and more.
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Client::new will automatically prepend your bot token with "Bot ", which is
    // a requirement by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Error creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
