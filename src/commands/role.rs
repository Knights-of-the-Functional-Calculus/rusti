use serenity::prelude::Context;
use serenity::{
    framework::standard::{macros::command, macros::group, Args, CommandResult},
    model::channel::Message,
};

use crate::util::travis::*;

group!({
    name: "role",
    options: {},
    commands: [iknow],
});

#[command]
fn iknow(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(&ctx, "What?").expect("Message not sent.");
        return Ok(());
    }

    let skill: String = args.single::<String>()?.to_lowercase();
    const PROGRAMMING_LANGUAGES: [&str; 2] = ["python", "javascript"];
    debug!("iknow:\t{:?}", msg.author.name);
    if PROGRAMMING_LANGUAGES.contains(&skill.as_str()) {
        let lock = msg.guild(&ctx).unwrap().clone();
        let guild = lock.read();
        let role = guild.role_by_name(&skill.as_str()).unwrap();
        if msg.author.has_role(&ctx, guild.id, role).unwrap() {
            msg.reply(&ctx, "I know quit bragging...")
                .expect("Message not sent.");
        } else {
            if let Ok(repo) = args.single::<String>() {
                let body: &serde_json::Value = &json!({
                 "request": {
                 "message": format!("User: {}#{}, Language: {}, Repo: {}", msg.author.name, msg.author.discriminator, skill, repo),
                 "branch":"master",
                 "config": {
                   "env": {
                     "REPO": repo,
                     "user_id": msg.author.id,
                     "guild_id": guild.id,
                     "role": skill
                   }
                  }
                }});
                post_travis_repo(
                    "repo",
                    "Knights-of-the-Functional-Calculus",
                    "code-skill-validator-python",
                    "requests",
                    Some(body),
                );

                msg.reply(
                    &ctx,
                    "It will take a while to test your code. I'll will ping you in a bit.",
                )
                .expect("Message not sent.");
            } else {
                msg.reply(&ctx,
                    &format!("You will need to to pass the tests here: https://github.com/Knights-of-the-Functional-Calculus/code-skill-validator-{0}. Send me your git repo like so: ```~iknow {0} <git repo>```", skill)
                ).expect("Message not sent.");
            }
        }
    } else {
        msg.reply(&ctx, "Oh shit, we got a badass over here...")
            .expect("Message not sent.");
    }

    Ok(())
}
