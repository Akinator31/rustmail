use serenity::all::{Context, Message};
use crate::config::Config;
use crate::errors::ModmailResult;

pub async fn poll(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let _ = msg.reply(ctx, format!("**Poll** {}**", msg.author.name)).await;

    Ok(())
}
