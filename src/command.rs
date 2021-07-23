use std::{collections::HashSet, ops::Range, str::FromStr};
use serenity::{client::Context, model::{channel::Message, id::RoleId}};


pub struct CommandInfo{
    commands: Vec<String>,
    args_range: Range<usize>,
    required_permissions: HashSet<Permission>,
    required_roles: HashSet<String>
}

pub struct CommandManager {
    pub command_infos: Vec<CommandInfo>
}

impl CommandManager {
    pub fn new() -> CommandManager {
        CommandManager {command_infos: vec![]}
    }

    // adds a new command that can be called by _commands_ and has an _args_range_
    // with certain required permisions and/or roles, and takes the callback to call
    // slices used here to make registering a new command easier
    pub fn register(&mut self, commands: &[&str], min_args: usize, max_args: usize, required_permissions: &[Permission], required_roles: &[&str]) {
        self.command_infos.push(CommandInfo {
            commands: commands.into_iter().map(|s| {s.to_string()}).collect::<Vec<String>>(),
            args_range: min_args..(max_args+1),
            required_permissions: required_permissions.into_iter().map(|p| {*p}).collect::<HashSet<Permission>>(),
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
            println!("Args: {}", args_string);
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

    fn permisions_valid(&self, command_indices: (usize, usize), ctx: &Context, msg: &Message) {
        // ctx.http.delete_message(channel_id, message_id) TODO
    }

    async fn roles_valid(&self, command_indices: (usize, usize), ctx: &Context, msg: &Message) -> serenity::Result<()> {
        let guild_id = msg.guild(ctx).await.expect("Guild Error").id;

        for role in &self.command_infos[command_indices.0].required_roles {
            let role = RoleId::from_str(role).expect("RoleID Parse Error");

            if !msg.author.has_role(ctx, guild_id, role).await? {
                msg.reply(ctx, "role not available TODO").await?;
            }
        }

        Ok(())
    }

    async fn execute_cmd(&self, command_indices: (usize, usize), args: &str, ctx: &Context, msg: &Message) -> serenity::Result<()> {
        match self.command_infos[command_indices.0].commands[command_indices.1].as_str() {
            "ping" | "test" => {
                println!("Pinging..");
                ping(args, ctx, msg).await
            },
            "help" | "info" => {
                println!("Helping..");
                help(args, ctx, msg).await
            },
            _ => {
                panic!("Unreachable reached!");
            }

        }

        
    }

    pub async fn handle_messages(&self, ctx: &Context, msg: &Message) {
        println!("Handling messages");
        let cmd_option = CommandManager::get_command(msg);
        
        if cmd_option.is_some() {
            let cmd = cmd_option.unwrap();
            let cmd_indices_option = self.command_valid(&cmd);

            if cmd_indices_option.is_some() {
                let cmd_indices = cmd_indices_option.unwrap();
                let args = self.get_args(cmd_indices, msg);

                if self.args_valid(cmd_indices, &args) {
                    self.execute_cmd(cmd_indices, &args, ctx, msg).await.expect("exec cmd error");
                }
            
            }
        }

        

    }
}

async fn ping(args: &str, ctx: &Context, msg: &Message) -> serenity::Result<()> {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

async fn help(args: &str, ctx: &Context, msg: &Message) -> serenity::Result<()> {
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
            _ => {
                panic!("Unreachable reached!");
            }
        }
    }

    Ok(())
}


#[allow(dead_code)]
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub enum Permission {
    // General permissions
    Administrator,
    ViewAuditLog,
    ViewServerInsights,
    ManageServer,
    ManageRoles,
    ManageChannels,
    KickMembers,
    BanMembers,
    CreateInstantInvite,
    ChangeNickname,
    ManageNicknames,
    ManageEmojis,
    ManageWebhooks,
    ViewChannels,

    // Text permissions
    SendMessages,
    SendTTSMessages,
    ManageMessages,
    EmbedLinks,
    AttachFiles,
    ReadMessageHistory,
    MentionEveryone,
    UseExternalEmojis,
    AddReactions,
    UseSlashCommands,

    // Voice permissions
    Connect,
    Speak,
    Video,
    MuteMembers,
    DeafenMembers,
    MoveMembers,
    UseVoiceActivity,
    PrioritySpeaker
}
