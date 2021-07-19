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

// dont forget to register command into vec of commands in cmd.rs
#[derive(Debug)]
enum Cmd {
    None,
    NotFound(String),
    UsageError(String),
    Ping,
    Help(String),
    GetMessages(u64),
    CreateChannel(String)
}

// get the command and do the arg checking here
fn get_cmd(msg: &Message) -> Cmd {
    let mut msg_chars = msg.content.as_str().chars();

    // check if message is a command
    if msg_chars.next() != Some('!') {
        Cmd::None

    } else {
        let mut cmd_string = String::new();

        // cmd_string is equal to start of message (after !) till first whitespace
        for ch in &mut msg_chars {
            if ch.is_whitespace() {
                break;
            } else {
                cmd_string.push(ch);
            }
        }

        let args: Vec<char> = msg_chars.collect();

        match cmd_string.as_str() {
            cmd::PING_COMMAND =>  {
                Cmd::Ping
            },
            
            cmd::HELP_COMMAND => {
                Cmd::Help(args.into_iter().collect())
            },

            cmd::GET_MESSAGES_COMMAND => {
                let number = args.into_iter().collect::<String>().parse();

                if number.is_err() || number.clone().unwrap() > 100 {
                    Cmd::UsageError("get_messages".to_string())
                } else {
                    Cmd::GetMessages(number.unwrap())
                }

            },

            cmd::CREATE_CHANNEL_COMMAND => {
                let name: String = args.into_iter().collect();
                
                if name.is_empty() {
                    Cmd::UsageError("create_channel".to_string())
                } else {
                    Cmd::CreateChannel(name)
                }

            },

            not_found => {
                Cmd::NotFound(not_found.to_string())
            }
        }

    }
}

// only do logic related to command here
#[async_recursion::async_recursion]
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
                        cmd::GET_MESSAGES_COMMAND => {
                            msg.reply(ctx, cmd::GET_MESSAGES_SUMMARY).await?;
                        },
                        cmd::CREATE_CHANNEL_COMMAND => {
                            msg.reply(ctx, cmd::CREATE_CHANNEL_SUMMARY).await?;
                        }
                        
                        _ => {} // not reachable
                    }
                }
            }
        },

        Cmd::GetMessages(number) => {
            let channel = msg.channel_id;
            let message_id = msg.id;

            let messages = channel.messages(ctx, |retriever| {
                retriever.before(message_id).limit(number)
            }).await?;

            println!("{:?}", messages.into_iter().map(|e| {e.content}).collect::<Vec<String>>());

            msg.reply(ctx, cmd::GET_MESSAGES_MESSAGE).await?;

        },

        Cmd::CreateChannel(name) => {
            msg.guild_id.unwrap().create_channel(ctx, |channel| {
                channel.name(name)
            }).await?;

            msg.reply_mention(ctx, cmd::CREATE_CHANNEL_MESSAGE).await?;
        },

        Cmd::UsageError(cmd_string) => {
            execute_cmd(ctx, msg, Cmd::Help(cmd_string)).await?;
        }

        Cmd::NotFound(cmd_string) => {
            msg.reply(ctx, format!("{}: {}", cmd_string, cmd::COMMAND_NOT_FOUND)).await?;
        },

        Cmd::None => {},
    }

    Ok(())
}


