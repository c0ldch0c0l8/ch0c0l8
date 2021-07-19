pub const COMMAND_STRINGS: [&str; 4] = ["ping", "help", "get_messages", "create_channel"];
pub const COMMAND_NOT_FOUND: &str = "Command not found.";  


pub const PING_COMMAND: &str = "ping";
pub const PING_SUMMARY: &str = "ping: The usual ping command.\nUSAGE: !ping";

pub const PING_MESSAGE: &str = "Pong!";


pub const HELP_COMMAND: &str = "help";
pub const HELP_SUMMARY: &str = "help: Displays general help or help for a specific command.\nUSAGE: !help <command> (if any)";

pub const HELP_MESSAGE: &str = "
Hello there, Human!
You have summoned me. Let's see about getting you what you need.
❓ Need technical help?
➡️ Post in the <#CHANNEL_ID> channel and other humans will assist you.
❓ Looking for the Code of Conduct?
➡️ Here it is: 
❓ Something wrong?
➡️ You can flag an admin with @admin
I hope that resolves your issue!
— HelpBot 🤖
";

// make it dynamic or sth 
// let help = format!("{:?} are all available commands.\n{}\n{}\n{}\n{}", 
//     COMMAND_STRINGS,
//     PING_SUMMARY,
//     HELP_SUMMARY,
//     GET_MESSAGES_SUMMARY,
//     CREATE_CHANNEL_SUMMARY
// );


pub const GET_MESSAGES_COMMAND: &str = "get_messages"; 
pub const GET_MESSAGES_SUMMARY: &str = "Gets any number between 0 and 100 messages sent in the channel.\nUSAGE: !get_messages <number> (between 0 and 100)";

pub const GET_MESSAGES_MESSAGE: &str = "Got messages [check]";


pub const CREATE_CHANNEL_COMMAND: &str = "create_channel";
pub const CREATE_CHANNEL_SUMMARY: &str = "Creates a text channel with a given name\nUSAGE: !create_channel <channel> (non-empty)";

// this cmd and prolly some others in the future will likely not have a reason to reply with a message
pub const CREATE_CHANNEL_MESSAGE: &str = "DONE";