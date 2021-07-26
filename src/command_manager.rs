use std::{collections::HashSet, ops::Range};
use serenity::{client::{EventHandler, Context}, model::channel::Message};
use serenity::model::id::{GuildId, ChannelId, UserId};

use crate::commands as cmd;

pub struct CommandInfo {
    commands: Vec<String>,
    args_range: Range<usize>,
    required_roles: HashSet<String>
}

pub struct CommandManager {
    pub command_infos: Vec<CommandInfo>
}

impl CommandManager {
    pub fn new() -> CommandManager {
        CommandManager {
            command_infos: vec![],
        }
    }

    // adds a new command that can be called by _commands_ and has an _args_range_
    // with certain required permisions and/or roles, and takes the callback to call
    // slices used here to make registering a new command easier
    pub fn register(&mut self, commands: &[&str], min_args: usize, max_args: usize, required_roles: &[&str]) {
        self.command_infos.push(CommandInfo {
            commands: commands.into_iter().map(|s| {s.to_string()}).collect::<Vec<String>>(),
            args_range: min_args..(max_args+1),
            required_roles: required_roles.into_iter().map(|s| {s.to_string()}).collect::<HashSet<String>>()
        });
    }

    fn get_command(msg: &Message) -> Option<String> {
        let mut command_string = "".to_string();

        let mut msg_iter = msg.content.chars().into_iter();
        let msg_first_char = msg_iter.next();

        if msg_first_char.is_none()  {
            None
        } else if !(msg_first_char.unwrap() == '!') {
            None
        } else {

            for msg_char in msg_iter {
                if msg_char.is_whitespace() {
                    break
                } else {
                    command_string.push(msg_char);
                }
            }

            println!("Command: {}", command_string);
            Some(command_string)
        }
    }

    fn command_valid(&self, command_str: &str) -> Option<(usize, usize)> {
        for (command_info_index, _) in self.command_infos.iter().enumerate() {
            for (command_index, command) in self.command_infos[command_info_index].commands.iter().enumerate() {
                if command_str == command {
                    println!("Command indices: ({}, {})", command_info_index, command_index);
                    return Some((command_info_index, command_index));
                }
            }
        }

        None
    } 

    // will panic if index is out of range. consider using get() and doing actual error handling lol
    fn get_args(&self, command_indices: (usize, usize), msg: &Message) -> String {

        let args_start = self.command_infos[command_indices.0].commands[command_indices.1].len() + 2; // plus 2 to count the '!' and the space between the command and arg
        let args_string = msg.content.chars().skip(args_start).collect::<String>();

        if args_string.is_empty() {
            println!("Args: \"\"");
        } else {
            println!("Args: {}", args_string)
        }

        args_string
    }

    fn args_valid(&self, command_indices: (usize, usize), args_str: &str) -> bool {
        let args_len = args_str.split_whitespace().into_iter().count();

        if self.command_infos[command_indices.0].args_range.contains(&args_len) {
            println!("Args valid: true");
            true
        } else {
            println!("Args valid: false");
            false
        }

    }

    async fn roles_valid(&self, command_indices: (usize, usize), ctx: &Context, msg: &Message) -> serenity::Result<bool> {
        let guild_id = msg.guild(ctx).await.expect("Guild Error").id;
        let all_roles = guild_id.roles(ctx).await?;
        let mut required_roles = vec![];

        for role in all_roles {
            if self.command_infos[command_indices.0].required_roles.contains(&role.1.name) {
                required_roles.push(role);
            }
        }

        for role in required_roles {
            
            if !msg.author.has_role(ctx, guild_id, role.0).await? {
                println!("Roles valid: false");

                return Ok(false);
            }
        }

        println!("Roles valid: true");

        Ok(true)
    }

    async fn execute_cmd(&self, command_indices: (usize, usize), args: &str, ctx: &Context, msg: &Message) -> serenity::Result<()> {
        match self.command_infos[command_indices.0].commands[command_indices.1].as_str() {
            "ping" | "test" => {
                println!("Pinging..");
                cmd::ping(args, ctx, msg).await
            },
            "help" | "info" => {
                println!("Helping..");
                cmd::help(args, ctx, msg).await
            },
            "delete_channel" => {
                println!("Deleting channel..");
                cmd::delete_channel(args, ctx, msg).await
            },
            "off" => {
                println!("Turning off..");
                cmd::off(args, ctx, msg).await
            }
            _ => {
                panic!("Unreachable reached!");
            }

        }

        
    }

    pub async fn handle_messages(&self, ctx: &Context, msg: &Message) -> serenity::Result<()> {
        if msg.author.id == UserId::from(865992990703091713) || msg.channel_id == ChannelId::from(868751991755669514) {
            return Ok(());
        }
        
        println!("Handling messages..");

        match CommandManager::get_command(msg) {
            // a command
            Some(cmd) => {
                match self.command_valid(&cmd) {
                    // command found
                    Some(cmd_indices) => {
                        let args = self.get_args(cmd_indices, msg);
                        

                        if self.args_valid(cmd_indices, &args) { // args valid
                            if self.roles_valid(cmd_indices, ctx, msg).await? { // roles valid
                                self.execute_cmd(cmd_indices, &args, ctx, msg).await

                            } else { // roles invalid
                                msg.reply(ctx, format!("You are not assigned the role(s) required for this command. Use `!help {}` for more info", cmd)).await?;
                                Ok(())
                                
                            }

                        } else { // args invalid
                            msg.reply(ctx, format!("Invalid arguments for `{0}`. Use `!help {0}` for more info.", cmd)).await?;
                            Ok(())

                        }
                    },
                    // command not found
                    None => {
                        msg.reply(ctx, format!("Command `{}` is not found. Use `!help` for more info.", cmd)).await?;
                        Ok(())

                    }
                }
            },
            // Not a command. just a message
            None => { Ok(()) }
        }
    }
}

#[serenity::async_trait]
impl EventHandler for CommandManager {
    async fn message(&self, ctx: Context, msg: Message) {
        match self.handle_messages(&ctx, &msg).await {
            Ok(()) => {
                // all good
            }, 
            Err(e) => {
                // serenity error (not from user input)
                println!("{}", e);
            }
        }
    }

    async fn ready(&self, ctx: Context, data_about_bot: serenity::model::prelude::Ready) {
        println!("Bot {} READY!", data_about_bot.user.name);
        
        match say_bot_info(&ctx, "BOT ON!").await {
            Ok(()) => {
                // all good
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

// DONT USE THIS FOR LOGGING LOL
pub async fn say_bot_info(ctx: &Context, content: &str) -> serenity::Result<()> {

    GuildId::from(866289778486673458)
        .channels(ctx)
        .await?
        .get(&ChannelId::from(868751991755669514))
        .expect("no bot-info channel")
        .say(ctx, content)
        .await?;

    Ok(())

}
