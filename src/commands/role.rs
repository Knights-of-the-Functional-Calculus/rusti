use serenity::{
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        help_commands,
        macros::{check, command, group, help},
        Args, CheckResult, CommandGroup, CommandOptions, CommandResult, DispatchError, HelpOptions,
        StandardFramework,
    },
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
    },
    utils::{content_safe, ContentSafeOptions},
};

use std::env;

#[command]
pub fn iknow(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let settings = if let Some(guild_id) = msg.guild_id {
        // By default roles, users, and channel mentions are cleaned.
        ContentSafeOptions::default()
            // We do not want to clean channal mentions as they
            // do not ping users.
            .clean_channel(false)
            // If it's a guild channel, we want mentioned users to be displayed
            // as their display name.
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let skill: String = args.single::<String>()?.to_lower();
    const PROGRAMMING_LANGUAGES: [String; 2] = ["python", "javascript"];
    if PROGRAMMING_LANGUAGES.contains(skill) {
        let guild = msg.guild();
        let role = guild.role_by_name(skill);
        if msg.author.has_role(&ctx.http, guild, role).unwrap() {
            msg.reply(&ctx.http, &format!("I know quit bragging..."));
        } else {
            let repo: String = args.single::<String>()?;
            if msg.is_private() && repo {
                let resp: serde_json::Value = reqwest::Client::new()
                    .post(&format!("https://api.travis-ci.com/repo/travis-ci%2FKnights-of-the-Functional-Calculus/code-skill-validator-{}", skill))
                    .header("Travis-API-Version", "3")
                    .header(
                        "Authorization",
                        &format!("token {}", &env::var("TRAVIS_TOKEN").unwrap()),
                    )
                    .json(&json!({
                     "request": {
                     "message": format!("User: {}#{}, Language: {}, Repo: {}", msg.author.name, msg.author.discriminator, skill, repo),
                     "branch":"master",
                     "config": {
                       "env": {
                         "REPO": repo
                       },
                       "script": "sh entrypoint.sh"
                      }
                    }}))
                    .send()?
                    .json()?;
                println!("{:#?}", resp);
                msg.reply(
                    &ctx.http,
                    &format!(
                        "It will take a while to test your code. I'll will ping you in a bit."
                    ),
                );
            } else {
                msg.reply(&ctx.http,
                    &format!("You will need to to pass the tests here: https://github.com/Knights-of-the-Functional-Calculus/code-skill-validator-{}. Slide into my DMs and send me your git repo with the same command <3", skill)
                );
            }
        }
    } else {
        msg.reply(&ctx.http, &format!("Oh shit, we got a badass over here..."));
    }

    Ok(())
}
