mod cmd;
use std::env;

use serenity::prelude::*;
use serenity::model::channel::Message;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let cmd = get_cmd(&msg);

        println!("CMD: {:?}. MESSAGE: {} > {}", cmd, msg.author.name, msg.content_safe(&ctx).await);

        execute_cmd(&ctx, &msg, cmd).await.expect("Executing command error");
    }

    async fn ready(&self, _ctx: Context, data_about_bot: serenity::model::prelude::Ready) {
        println!("Bot {} READY!", data_about_bot.user.name);
    }

     
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Environment Variable for DISCORD_TOKEN NOT Set");
    let handler = Handler;
    
    let mut client = Client::builder(token).event_handler(handler).await.expect("Error Creating Client");
    

    if let Err(e) = client.start().await {
        println!("Error While Running Client: {}", e);
    }
}

#[derive(Debug)]
enum Cmd {
    None,
    NotFound(String),
    Ping,
    Help(String),
}


fn get_cmd(msg: &Message) -> Cmd {
    // get message as an iter over chars
    let mut msg_chars = msg.content.as_str().chars();

    // check if message is a command
    if msg_chars.next() != Some('!') {
        Cmd::None

    } else {
        // the command as a string
        let mut cmd_string = String::new();

        // is equal to start of message (after !) till first whitespace
        for ch in &mut msg_chars {
            if ch.is_whitespace() {
                break;
            } else {
                cmd_string.push(ch);
            }
        }

        // match the command
        match cmd_string.as_str() {
            cmd::PING_COMMAND =>  {
                Cmd::Ping
            },
            
            cmd::HELP_COMMAND => {
                let args_first_char = msg_chars.next();

                if args_first_char == None {
                    Cmd::Help("".to_string())

                } else {
                    let mut args = String::new();
                    args.push(args_first_char.unwrap());
                    args.push_str(msg_chars.collect::<String>().as_str());

                    Cmd::Help(args)
                }
            },

            not_found => {
                Cmd::NotFound(not_found.to_string())
            }
        }

    }
}

async fn execute_cmd(ctx: &Context, msg: &Message, cmd: Cmd) -> serenity::Result<()> {
    match cmd {
        Cmd::Ping => {
            msg.reply(ctx, cmd::PING_MESSAGE).await?;
        },

        Cmd::Help(args) => {
            if args.is_empty() {
                msg.reply(ctx, cmd::HELP_MESSAGE).await?;

            } else {
                // if command not found
                if !cmd::COMMAND_STRINGS.contains(&args.as_str()) {
                    msg.reply(ctx, format!("{}: {}", args, cmd::COMMAND_NOT_FOUND)).await?;
                } else {
                    // else match the command with its summary
                    match args.as_str() {
                        cmd::PING_COMMAND => {
                            msg.reply(ctx, cmd::PING_SUMMARY).await?;
                        },
                        cmd::HELP_COMMAND => {
                            msg.reply(ctx, cmd::HELP_SUMMARY).await?;
                        },
                        _ => {} // not reachable
                    }
                }
            }
        },

        Cmd::NotFound(cmd_string) => {
            msg.reply(ctx, format!("{}: {}", cmd_string, cmd::COMMAND_NOT_FOUND)).await?;
        },

        Cmd::None => {},
    }

    Ok(())
}
