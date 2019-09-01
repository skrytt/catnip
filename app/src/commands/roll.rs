use rand::{Rng, thread_rng};

use serenity::{
    prelude::*,
    model::prelude::*,
    framework::standard::{
        CommandResult,
        CommandError,
        macros::command,
    },
    utils::MessageBuilder,
};

#[command]
/// Roll a number of same-sided dice and send a response with the result.
fn roll(context: &mut Context, msg: &Message) -> CommandResult {
    debug!("roll command handler called");

    let dice = match parse_roll(msg) {
        Ok(value) => value,
        Err(_) => return Err(CommandError(
            format!("Couldn't parse this message as dice: {:?}", msg.content)
            .to_string()
        )),
    };

    let mut rolled_value: u32 = 0;
    for _ in 0..dice[0] {
        rolled_value += roll_die(&dice[1]);
    };

    let response = MessageBuilder::new()
        .push_bold_safe(&msg.author)
        .push(" rolls a fistful of dice with the result: ")
        .push_bold(rolled_value)
        .build();

    if let Err(why) = msg.channel_id.say(&context.http, &response) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}

/// Transforms a message into a roll; e.g, for n * roll of m sided die:
///     !roll ndm -> [n, m]
fn parse_roll(msg: &Message) -> Result<[u32; 2], ()> {
    debug!("received: {:?}", msg);
    let args: Vec<&str> = msg.content.split(' ').collect();
    if args.len() != 2 {
        return Err(());
    }

    let roll: Vec<&str> = args[1].split('d').collect();
    let roll_len = roll.len();
    let (num_dice, num_sides) = match roll_len {
        1 => {
            let num_dice = 1;
            let num_sides = match atoi(roll[0]) {
                Ok(value) => {
                    if value < 101 { value } else { return Err(()) }
                },
                Err(_) => return Err(()),
            };
            (num_dice, num_sides)
        },
        2 => {
            let num_dice = if roll[0].is_empty() { 1 } else { 
                match atoi(roll[0]) {
                    Ok(value) => {
                        if value < 100 { value } else { return Err(()) }
                    },
                    Err(_) => return Err(()),
                }
            };
            let num_sides = match atoi(roll[1]) {
                Ok(value) => {
                    if value < 101 { value } else { return Err(()) }
                },
                Err(_) => return Err(()),
            };
            (num_dice, num_sides)
        },
        _ => return Err(())
    };
    Ok([num_dice, num_sides])
}

/// Generates random number based on value of die given
fn roll_die(d: &u32) -> u32 {;
    thread_rng().gen_range(1, *d + 1)
}

/// Convert string to a u32
fn atoi(s: &str) -> Result<u32, ()> {
    match s.to_string().parse::<u32>() {
        Ok(value) => Ok(value),
        Err(_) => return Err(()),
    }
}
