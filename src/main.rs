use serenity::{
    model::{channel::Message},
    prelude::*,
    framework::standard::macros::{command, group},
    framework::standard::{CommandResult, StandardFramework},

};
use std::sync::atomic::{AtomicUsize, Ordering};
use dotenv::var;

const PREFIX: &str = "+";

#[derive(Default)]
struct Global {
    count: AtomicUsize,
}

struct GlobalKey;

impl TypeMapKey for GlobalKey {
    type Value = Global;
}

#[group]
#[commands(ping, please_say, increase)]
struct General;

#[tokio::main]
async fn main() {
    let token = var("DISCORD_TOKEN").unwrap();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(PREFIX))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token).framework(framework).await.expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<GlobalKey>(Global::default());
    }

    client.start().await.unwrap()
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, format!("ping{}", PREFIX)).await?;
    Ok(())
}

#[command]
async fn please_say(ctx: &Context, msg: &Message) -> CommandResult {
    let content = msg.content.split_once("please_say").unwrap().1;
    msg.channel_id.say(&ctx.http, content.trim_start()).await?;
    Ok(())
}

#[command]
async fn increase(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let global = data.get::<GlobalKey>().unwrap();
    let val = global.count.fetch_add(1, Ordering::SeqCst) + 1;
    msg.channel_id.say(&ctx.http, format!("count: {}", val)).await?;
    Ok(())
}

