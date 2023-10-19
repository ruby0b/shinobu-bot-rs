use crate::commands::*;
use crate::prelude::*;
use poise::serenity_prelude as serenity;

pub mod consts;
pub mod prelude;
pub mod types;
pub mod commands {
    pub mod help;
    pub mod misc;
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![help::help(), misc::age()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(COMMAND_PREFIX.to_owned()),
                ..Default::default()
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    serenity::GuildId(
                        std::env::var("DISCORD_GUILD")
                            .expect("missing DISCORD_GUILD")
                            .parse::<u64>()
                            .expect("invalid DISCORD_GUILD"),
                    ),
                )
                .await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
