mod command;
use command::CommandManager;

use std::env;

use serenity::prelude::*;
use serenity::model::channel::Message;

#[serenity::async_trait]
impl EventHandler for CommandManager {
    async fn message(&self, ctx: Context, msg: Message) {
        self.handle_messages(&ctx, &msg).await;
    }

    async fn ready(&self, _ctx: Context, data_about_bot: serenity::model::prelude::Ready) {
        println!("Bot {} READY!", data_about_bot.user.name);
    }

     
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Environment Variable for DISCORD_TOKEN NOT Set");
    
    let mut command_manager = CommandManager::new();

    // register command here, in exec_command, and in help
    // consider adding a command_manager.validate() that checks 
    // that each command is set up and has provided all info
    // necessary. also check that roles in all commands are valid
    command_manager.register(&["ping", "test"], 0, 0, &[], &[]);
    command_manager.register(&["help", "info"], 0, 1, &[], &[]);
    command_manager.register(&["delete_channel"], 0, 0, &[], &["Admin"]);
    

    let mut client = Client::builder(token).event_handler(command_manager).await.expect("Error Creating Client");
    
    
    if let Err(e) = client.start().await {
        println!("Error While Running Client: {}", e);
    }
}


