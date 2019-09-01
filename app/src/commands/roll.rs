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
fn roll(context: &mut Context, msg: &Message) -> CommandResult {
    debug!("roll20 command handler called");

    dice = parse_roll(msg);
    if dice.is_err() {
        error!("Error parsing roll: {:?}", dice)
    }

    rolled_value: u32 = 0
    for value in 1..dice[0] {
        rolled_value += roll_die(dice[1])
    }

    let response = MessageBuilder::new()
        .push_bold_safe(&msg.author)
        .push(
            format!(" rolls {:?} with the result: ", msg.content.split(' ')[1])
        )
        .push_bold(rolled_value)
        .build();

    if let Err(why) = msg.channel_id.say(&context.http, &response) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}

/// Transforms a message into a roll; e.g, for n * roll of m sided die:
///     !roll ndm -> [n, m]
fn parse_roll(msg: &Message) -> [u32, 2] {
    debug!("received: {:?}", msg);
    roll = msg.content.split(' ')[1].split('d').collect();

    if dice.len() == 2 {
        let dice: [u32, 2] = [
            roll[0].parse().unwrap(), 
            roll[1].parse().unwrap()
        ]
    } else if dice.len() == 1 {
        let dice: [u32, 2] = [
            1
            roll[1].parse().unwrap()
        ]
    } else {
        error!("Too many dice for this version!: {:?}", dice);
    }
}

/// Generates random number based on value of die given
fn roll_die(d: &u32) -> u32 {
    thread_rng().gen_range(1, d+=1);
}