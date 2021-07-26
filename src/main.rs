mod command_manager;
use command_manager::CommandManager;

mod commands;

use serenity::prelude::*;

use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Environment Variable for DISCORD_TOKEN NOT Set");
    
    let mut command_manager = CommandManager::new();

    // register command here, in exec_command, and in help
    // consider adding a command_manager.validate() that checks 
    // that each command is set up and has provided all info
    // necessary. also check that roles in all commands are valid
    command_manager.register(&["ping", "test"], 0, 0, &[]);
    command_manager.register(&["help", "info"], 0, 1, &[]);
    command_manager.register(&["delete_channel"], 0, 0, &["Admin"]);
    command_manager.register(&["off"], 0, 0, &["Admin"]);

    
    let mut client = Client::builder(token)
        .event_handler(command_manager)
        .await
        .expect("client creation");

    if let Err(e) = client.start().await {
        println!("Error While Running Client: {}", e);
    }
}


