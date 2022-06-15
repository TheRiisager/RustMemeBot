use std::env;
use serenity::{
    model::gateway:: GatewayIntents,
    prelude::*,
};

mod discord;
mod db;

#[tokio::main]
async fn main() {
    
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    
    let mut client = Client::builder(&token, GatewayIntents::union(GatewayIntents::GUILD_MESSAGES, GatewayIntents::MESSAGE_CONTENT))
        .event_handler(discord::Handler)
        .await
        .expect("Err creating client");
    
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}