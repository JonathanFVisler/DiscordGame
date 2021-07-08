use crate::Global;
use serenity::{
    model::channel::Message,
    prelude::*,
};
use std::sync::Arc;

pub struct CmdContext {
    global: Arc<Global>,
    cxt: Context,
    msg: Message,
}

impl CmdContext {
    pub fn new(global: Arc<Global>, cxt: Context, msg: Message) -> Option<Self> {
        if msg.content.len() <= 1 || !msg.content.starts_with(crate::PREFIX) {
            return None;
        }
        Some(CmdContext { global, cxt, msg })
    }

    pub fn global(&self) -> &Global {
        &self.global
    }

    pub fn cmd(&self) -> &str {
        self.msg.content[1..].split_whitespace().next().unwrap()
    }

    pub fn args_raw(&self) -> &str {
        &self.msg.content[self.cmd().len() + 1..]
    }

    pub async fn respond(&self, res: impl AsRef<str>) {
        if let Err(e) = self.msg.channel_id.say(&self.cxt.http, res.as_ref()).await {
            println!("Failed to respond: {}\nmsg: {:?}", e, self.msg)
        }
    }
}
