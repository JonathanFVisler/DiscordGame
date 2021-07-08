use serenity::{
    model::{channel::Message},
    prelude::*,
    framework::standard::macros::{command, group},
    framework::standard::{CommandResult, StandardFramework},

};
use std::sync::Mutex;
use rusqlite::{Connection};
use dotenv::var;

const PREFIX: &str = "+";

struct DB;

impl TypeMapKey for DB {
    type Value = Mutex<Connection>;
}

#[group]
#[commands(ping, please_say)]
struct General;

#[tokio::main]
async fn main() {
    let token = var("DISCORD_TOKEN").unwrap();
    let db_path = var("DATABASE_PATH").unwrap();

    let db = Connection::open(db_path).unwrap();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(PREFIX))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token).framework(framework).await.expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<DB>(Mutex::new(db));
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
