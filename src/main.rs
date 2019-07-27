use serenity::client::Client;
use serenity::framework::standard::{macros::group, StandardFramework};
use serenity::prelude::EventHandler;

use serenity::prelude::Context;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, gateway::Ready},
};

mod commands;
use crate::commands::role::*;

mod util;
use crate::util::travis::*;

use std::env;

#[macro_use]
extern crate serde_json;

group!({
    name: "general",
    options: {},
    commands: [ping],
});

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, context: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        for guild in ready.guilds {
            guild
                .id()
                .create_role(&context.http, |r| r.hoist(true).name("python"));
        }
        post_travis_repo(
            "repo",
            "Knights-of-the-Functional-Calculus",
            "code-skill-validator-python",
            "activate",
            None,
        );
    }
}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
            .group(&GENERAL_GROUP)
            .group(&ROLE_GROUP),
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;
    println!("ping:\t{:?}", msg.author.name);

    Ok(())
}
