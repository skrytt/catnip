use rand::{Rng, thread_rng};

use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        macros::command,
    },
    utils::MessageBuilder,
};

#[command]
/// Roll 1d20 and send a response with the result.
async fn roll20(context: &Context, msg: &Message) -> CommandResult {
    debug!("roll20 command handler called");

    // The RNG function produces values in the range 0 to 20-1, so
    // add 1 to bring the range into the expected range.
    let rolled_value: i32 = thread_rng().gen_range(0, 20) + 1;

    let response = MessageBuilder::new()
        .push_bold_safe(&msg.author)
        .push(" rolls 1d20 with the result: ")
        .push_bold(rolled_value)
        .build();

    if let Err(why) = msg.channel_id.say(&context.http, &response) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
