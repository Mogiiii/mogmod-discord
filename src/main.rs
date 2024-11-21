use std::{env::var, time::SystemTime};
use dotenv::dotenv;
use log::info;
use poise::serenity_prelude as serenity;

mod backend;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
}

// /// Displays your or another user's account creation date
// #[poise::command(slash_command, prefix_command)]
// async fn age(
//     ctx: Context<'_>,
//     #[description = "Selected user"] user: Option<serenity::User>,
// ) -> Result<(), Error> {
//     let u = user.as_ref().unwrap_or_else(|| ctx.author());
//     let response = format!("{}'s account was created at {}", u.name, u.created_at());
//     ctx.say(response).await?;
//     Ok(())
// }

#[tokio::main]
async fn main() {
    dotenv().ok();

    env_logger::init();

    let token = var("DISCORD_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                })
            })
        })
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            info!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            let g = new_message.guild(&ctx.cache).unwrap().clone();
            let c = &g.channels[&new_message.channel_id];
            let ets = match new_message.edited_timestamp {
                None => None,
                Some(t) => Some(SystemTime::from(t.to_utc())),
            };
            let msg = backend::Message {
                id: new_message.id.get(),
                content: new_message.content.clone(),
                timestamp: SystemTime::from(new_message.timestamp.to_utc()),
                user_id: new_message.author.id.get(),
                user_name: new_message.author.name.clone(),
                guild_id: new_message.guild_id.unwrap().get(),
                guild_name: g.name.clone(),
                channel_id: new_message.channel_id.get(),
                channel_name: c.name.clone(), 
                edited_timestamp: ets,
            };
            backend::update_message(msg).await?
        }
        serenity::FullEvent::UserUpdate { old_data: _, new } => {
            
        }
        serenity::FullEvent::MessageDelete { channel_id: _, deleted_message_id, guild_id: _ } => {
            
        }
        serenity::FullEvent::GuildDelete { incomplete, full: _ } => {

        }
        serenity::FullEvent::ChannelDelete { channel, messages } => {

        }
        _ => {}
    }
    Ok(())
}
