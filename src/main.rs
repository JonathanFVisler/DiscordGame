mod cmd_cxt;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use cmd_cxt::CmdContext;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

const DISCORD_TOKEN: &str = "ODYxOTc0NjM3NTg4MjUwNjY1.YORmcA.MOwlxAYX7KOqu-E-VP77JpGjkZc";
const PREFIX: char = '+';

pub struct Global {
    count: AtomicUsize,
}

struct Handler(Arc<Global>);

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, cxt: Context, message: Message) {
        if let Some(cxt) = CmdContext::new(self.0.clone(), cxt, message) {
            run_command(cxt).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let global = Arc::new(Global {
        count: AtomicUsize::new(0),
    });
    let mut client = Client::builder(DISCORD_TOKEN).event_handler(Handler(global)).await.expect("Err creating client");

    client.start().await.unwrap()
}

async fn run_command(cxt: CmdContext) {
    match cxt.cmd() {
        "ping" => cxt.respond(format!("pong{}", PREFIX)).await,
        "please_say" => cxt.respond(cxt.args_raw()).await,
        "increase" => {
            let val = cxt.global().count.fetch_add(1, Ordering::SeqCst) + 1;
            cxt.respond(format!("count: {}", val)).await;
        },
        cmd => cxt.respond(format!("Unknown command `{}`", cmd)).await,
    }
}
