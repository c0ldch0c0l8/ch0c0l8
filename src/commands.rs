use serenity::prelude::*;
use serenity::model::channel::Message;
use crate::command_manager::say_bot_info;
use std::process::exit;


pub async fn ping(_: &str, ctx: &Context, msg: &Message) -> serenity::Result<()> {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

pub async fn help(args: &str, ctx: &Context, msg: &Message) -> serenity::Result<()> {
    if args == "" {
        msg.reply(ctx, "General Help; list commands here TODO").await?;
    } else {
        match args {
            "ping" | "test" => {
                msg.reply(ctx, "ping/test help").await?;
            },
            "help" | "info" => {
                msg.reply(ctx, "help/info help").await?;
            },
            "delete_channel" => {
                msg.reply(ctx, "delete_channel help").await?;
            },
            "off" => {
                msg.reply(ctx, "off help").await?;
            },
            _ => {
                panic!("Unreachable reached!");
            }
        }
    }

    Ok(())
}

pub async fn delete_channel(_: &str, ctx: &Context, msg: &Message) -> serenity::Result<()> {
    msg.channel_id.delete(ctx).await?;

    Ok(())
}

pub async fn off(_: &str, ctx: &Context, msg: &Message) -> serenity::Result<()> {
    msg.reply(ctx, "BOT OFF!").await?;
    say_bot_info(ctx, "BOT OFF!").await?;

    exit(0);
}
